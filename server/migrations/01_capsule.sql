CREATE TABLE IF NOT EXISTS public.users (
    id character varying NOT NULL PRIMARY KEY,
    name character varying NOT NULL
);

ALTER TABLE public.users OWNER TO postgres;

CREATE TABLE IF NOT EXISTS public.capsules (
    id character varying NOT NULL PRIMARY KEY,
    name character varying,
    content character varying,
    deadline timestamp with time zone,
    author_id character varying NOT NULL,
    created_at timestamp with time zone NOT NULL,
    CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES public.users(id) ON DELETE CASCADE
);

ALTER TABLE public.capsules OWNER TO postgres;


