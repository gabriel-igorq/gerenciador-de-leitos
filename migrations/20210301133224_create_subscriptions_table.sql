-- Create Subscriptions Table
CREATE TABLE unidadeSaude(
	   id uuid NOT NULL,
	   PRIMARY KEY (id),
	   email TEXT NOT NULL UNIQUE,
	   nome TEXT NOT NULL,
	   tipo TEXT NOT NULL,
	   municipio TEXT NOT NULL,
	   subscribed_at timestamptz NOT NULL
);

