interface CreatedCapsuleResponse {
  capsule: CapsuleResponse;
  key: string;
}

interface CapsuleResponse {
  id: string;
  name: string;
  content: string;
  author: string;
  deadline: string;
  createdAt: string;
}

export interface Capsule {
  id: string;
  name: string;
  content: string;
  author: string;
  deadline: Date;
  createdAt: Date;
}

export interface CreateCapsule {
  name: string;
  content: string;
  author: string;
  deadline: string;
}

const BASE_PATH = "http://localhost:3000";

const parseResponse = (response: CapsuleResponse): Capsule => {
  return {
    id: response.id,
    name: response.name,
    content: response.content,
    author: response.author,
    deadline: new Date(response.deadline),
    createdAt: new Date(response.createdAt),
  };
};

export const fetchCapsule = async (capsuleId: string, key: string): Promise<Capsule> => {
  return new Promise((resolve, reject) => {
    fetch(`${BASE_PATH}/capsule/${capsuleId}?sleutel=${key}`).then((body) => {
      if (body.status === 404) {
        reject({
          error: "Capsule not found...",
          errorCode: "CAPSULE_NOT_FOUND",
        });
        return;
      }

      if (body.status === 403) {
        body.json().then((data) =>
          reject({
            error: "Capsule not ready yet...",
            errorCode: "CAPSULE_NOT_READY",
            data: parseResponse(data),
          })
        );
        return;
      }

      body.json().then((data) => {
        resolve(parseResponse(data));
      });
    });
  });
};

export const createCapsule = async (
  capsule: CreateCapsule
): Promise<CreatedCapsuleResponse> => {
  return new Promise((resolve, reject) => {
    fetch(`${BASE_PATH}/capsule`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(capsule),
    }).then((body) => {
      body.json().then((data) => {
        resolve(data);
      });
    });
  });
};
