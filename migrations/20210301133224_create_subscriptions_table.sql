-- Create Subscriptions Table
CREATE TABLE unidadeSaude(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   email TEXT NOT NULL UNIQUE,
	   nome TEXT NOT NULL,
	   tipo TEXT NOT NULL,
	   municipio TEXT NOT NULL
);

CREATE TABLE leito(
		id uuid NOT NULL,
		PRIMARY KEY (id),
		tipo TEXT NOT NULL,
		situacao TEXT NOT NULL,
		unidade_id uuid NOT NULL,
		FOREIGN KEY (unidade_id) REFERENCES unidadeSaude (id)
);

CREATE TABLE paciente(
		id uuid NOT NULL,
		nome TEXT NOT NULL,
		sexo TEXT NOT NULL,
		idade TEXT NOT NULL,
		email TEXT NOT NULL,
		telefone TEXT NOT NULL,
		covid_19 TEXT NOT NULL,
		leito_id uuid NOT NULL,
		FOREIGN KEY (leito_id) REFERENCES leito (id)
);