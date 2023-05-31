CREATE TABLE Host(
    id SERIAL,
    os TEXT NOT NULL,
    cpu TEXT NOT NULL,
    memory BIGINT UNSIGNED NOT NULL,
    CONSTRAINT Host_pk PRIMARY KEY (id)
);

CREATE TABLE Report(
    id SERIAL,
    `timestamp` BIGINT UNSIGNED NOT NULL,
    `commit` TEXT NOT NULL,
    host_id BIGINT UNSIGNED NOT NULL,
    CONSTRAINT Report_pk PRIMARY KEY (id),
    CONSTRAINT Report_FK FOREIGN KEY (host_id) REFERENCES Host(id) ON DELETE RESTRICT
);

CREATE TABLE Metric (
	id SERIAL,
	name TEXT NOT NULL,
	`time` BIGINT UNSIGNED NOT NULL,
    report_id BIGINT UNSIGNED NOT NULL,
	CONSTRAINT Metric_pk PRIMARY KEY (id),
	CONSTRAINT Metric_FK FOREIGN KEY (report_id) REFERENCES Report(id) ON DELETE CASCADE ON UPDATE CASCADE
);