CREATE USER 'azgeda'@'%' IDENTIFIED BY 'freetopGbrothers';
CREATE USER 'orwellian'@'%' IDENTIFIED BY 'orcasarecoolanimals';
CREATE USER 'sps_api'@'%' IDENTIFIED BY 'belugasaredopetoo';

GRANT ALL PRIVILEGES ON Apollonius.* TO 'azgeda'@'%';
GRANT ALL PRIVILEGES ON Apollonius.* TO 'orwellian'@'%';

GRANT SELECT, INSERT, UPDATE ON Apollonius.* TO 'sps_api'@'%';