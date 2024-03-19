CREATE EXTENSION IF NOT EXISTS unaccent;

-- Criando um novo dicionário de busca de texto usando a extensão unaccent
ALTER TEXT SEARCH DICTIONARY unaccent (RULES='unaccent');

-- Criando uma nova configuração de busca de texto chamada 'pessoas'
CREATE TEXT SEARCH CONFIGURATION pessoas (COPY = portuguese);

-- Alterando o mapeamento de palavras na configuração 'pessoas' para usar a função unaccent
ALTER TEXT SEARCH CONFIGURATION pessoas ALTER MAPPING FOR hword, hword_part, word WITH unaccent, portuguese_stem;

CREATE SEQUENCE exemplo_id_seq START 1000;


CREATE OR REPLACE FUNCTION ARRAY_TO_STRING_IMMUTABLE(
  arr INTEGER[],
  sep TEXT
) RETURNS TEXT IMMUTABLE PARALLEL SAFE LANGUAGE SQL AS $$  
  SELECT ARRAY_TO_STRING(arr::TEXT[], sep); 
$$;

CREATE TABLE AUTH (
    ID INTEGER PRIMARY KEY NOT NULL,
    NOME VARCHAR(100) NOT NULL
);


CREATE TABLE USUARIO (
    ID UUID PRIMARY KEY,
    CPF VARCHAR(20) NOT NULL UNIQUE,
    NOME VARCHAR(100) NOT NULL,
    FK_AUTH_ID INTEGER NOT NULL,
    search TSVECTOR GENERATED ALWAYS AS(
        TO_TSVECTOR('pessoas', Nome || ' ' || CPF)
    ) STORED
);

CREATE TABLE MEGA (
    ID UUID PRIMARY KEY NOT NULL,
    DATA_ DATE NOT NULL,
    AMOUNT INTEGER NOT NULL,
    FK_USER_ID UUID REFERENCES USUARIO (ID) ON DELETE CASCADE NOT NULL
);


CREATE INDEX usuario_search_index ON USUARIO USING GIN(search);

CREATE TABLE APOSTA (
    ID INTEGER PRIMARY KEY DEFAULT nextval('exemplo_id_seq'),
    VEC INTEGER[5] NOT NULL,
    FK_USER_ID UUID NOT NULL,
    FK_MEGA_ID UUID NOT NULL,
    FOREIGN KEY (FK_USER_ID) REFERENCES USUARIO (ID) ON DELETE CASCADE,
    FOREIGN KEY (FK_MEGA_ID) REFERENCES MEGA (ID) ON DELETE CASCADE
);

INSERT INTO AUTH (ID,NOME) VALUES (1,'admin');

INSERT INTO AUTH (ID,NOME) VALUES (0,'user');

INSERT INTO USUARIO (ID, CPF, NOME, FK_AUTH_ID) VALUES ('018e4ef8-cd7c-7564-83f4-408710bb37ff', '00000000000', 'admin', 1);

CREATE FUNCTION array_contains_all(elements integer[], target integer[]) RETURNS BOOLEAN AS $$
DECLARE
    element integer;
BEGIN
    FOREACH element IN ARRAY elements
    LOOP
        IF NOT element = ANY(target) THEN
            RETURN FALSE;
        END IF;
    END LOOP;
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;
