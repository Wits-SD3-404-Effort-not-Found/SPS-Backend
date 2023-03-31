CREATE USER 'azgeda'@'%' IDENTIFIED BY 'freetopGbrothers';
CREATE USER 'orwellian'@'%' IDENTIFIED BY 'orcasarecoolanimals';
CREATE USER 'sps_api'@'%' IDENTIFIED BY 'belugasaredopetoo';

GRANT ALL PRIVILEGES ON database_name.* TO 'azgeda'@'%';
GRANT ALL PRIVILEGES ON database_name.* TO 'orwellian'@'%';

GRANT SELECT, INSERT, UPDATE ON database_name.* TO 'sps_api'@'%';