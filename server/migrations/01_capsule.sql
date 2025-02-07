--
-- PostgreSQL database dump
--

-- Dumped from database version 16.3
-- Dumped by pg_dump version 16.3

-- Started on 2024-07-10 18:08:21

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
-- SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 216 (class 1259 OID 16637)
-- Name: capsules; Type: TABLE; Schema: public; Owner: postgres
--

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

--
-- TOC entry 215 (class 1259 OID 16630)
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE IF NOT EXISTS public.users (
    id character varying NOT NULL PRIMARY KEY,
    name character varying NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- TOC entry 4640 (class 2606 OID 16643)
-- Name: capsules capsules_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

-- ALTER TABLE ONLY public.capsules
--     ADD CONSTRAINT capsules_pkey PRIMARY KEY (id);


--
-- TOC entry 4638 (class 2606 OID 16636)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

-- ALTER TABLE ONLY public.users
--     ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- TOC entry 4641 (class 2606 OID 16644)
-- Name: capsules capsules_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

-- ALTER TABLE ONLY public.capsules
--     ADD CONSTRAINT capsules_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.users(id);


-- Completed on 2024-07-10 18:08:21

--
-- PostgreSQL database dump complete
--

