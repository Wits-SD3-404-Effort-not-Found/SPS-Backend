CREATE DATABASE Apollonius;
USE Apollonius;

CREATE TABLE tblAccount (
  account_id INT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL,
  hashed_password VARCHAR(255) NOT NULL,
  username VARCHAR(255) NOT NULL,
  cell_number VARCHAR(255),
  profile_photo BLOB
);

CREATE TABLE tblHospital (
  hospital_id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE tblDiscipline (
  discipline_id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE tblSessionToken (
  session_token_id INT PRIMARY KEY AUTO_INCREMENT,
  account_id INT NOT NULL,
  token VARCHAR(255) NOT NULL,
  expiry_date DATE NOT NULL,
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblNotes (
  note_id INT PRIMARY KEY AUTO_INCREMENT,
  account_id INT NOT NULL,
  title VARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblProtocol (
  protocol_id INT PRIMARY KEY AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  content TEXT
);

CREATE TABLE tblLogbook (
  logbook_id INT PRIMARY KEY AUTO_INCREMENT,
  account_id INT NOT NULL,
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblEvents (
  event_id INT PRIMARY KEY AUTO_INCREMENT,
  account_id INT NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblRotation (
  rotation_id INT PRIMARY KEY AUTO_INCREMENT,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  hospital_id INT NOT NULL,
  discipline_id INT NOT NULL,
  FOREIGN KEY (hospital_id) REFERENCES tblHospital(hospital_id),
  FOREIGN KEY (discipline_id) REFERENCES tblDiscipline(discipline_id)
);

CREATE TABLE tblAssignedRotations (
  assigned_rotation_id INT PRIMARY KEY AUTO_INCREMENT,
  rotation_id INT NOT NULL,
  account_id INT NOT NULL,
  FOREIGN KEY (rotation_id) REFERENCES tblRotation(rotation_id),
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblSecurityQuestions (
    secques_id INT PRIMARY KEY AUTO_INCREMENT,
    question VARCHAR(255) NOT NULL
);

CREATE TABLE tblSecurityAnswers (
    secans_id INT PRIMARY KEY AUTO_INCREMENT,
    secques_id INT NOT NULL,
    account_id INT NOT NULL,
    answer VARCHAR(255) NOT NULL,
    FOREIGN KEY (secques_id) REFERENCES tblSecurityQuestions(secques_id),
    FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);
