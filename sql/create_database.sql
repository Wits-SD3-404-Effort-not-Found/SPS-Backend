CREATE DATABASE Apollonius;
USE Apollonius;


CREATE TABLE tblAccount (
  id INT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(255) NOT NULL,
  hashed_password VARCHAR(255) NOT NULL,
  username VARCHAR(255) NOT NULL,
  cell_number VARCHAR(255),
  profile_photo BLOB
);

CREATE TABLE tblHospital (
  id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE tblDiscipline (
  id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE tblSessionToken (
  id INT PRIMARY KEY AUTO_INCREMENT,
  account_id INT NOT NULL,
  token VARCHAR(255) NOT NULL,
  expiry_date DATE NOT NULL,
  FOREIGN KEY (account_id) REFERENCES tblAccount(id)
);

CREATE TABLE tblNotes (
  id INT PRIMARY KEY AUTO_INCREMENT,
  note_id INT NOT NULL,
  account_id INT NOT NULL,
  title VARCHAR(255) NOT NULL,
  content TEXT,
  FOREIGN KEY (account_id) REFERENCES tblAccount(id)
);

CREATE TABLE tblProtocol (
  id INT PRIMARY KEY AUTO_INCREMENT,
  protocol_id INT NOT NULL,
  title VARCHAR(255) NOT NULL,
  content TEXT
);

CREATE TABLE tblLogbook (
  id INT PRIMARY KEY AUTO_INCREMENT,
  logbook_id INT NOT NULL,
  account_id INT NOT NULL,
  FOREIGN KEY (account_id) REFERENCES tblAccount(id)
);

CREATE TABLE tblEvents (
  id INT PRIMARY KEY AUTO_INCREMENT,
  event_id INT NOT NULL,
  account_id INT NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  FOREIGN KEY (account_id) REFERENCES tblAccount(id)
);

CREATE TABLE tblRotation (
  id INT PRIMARY KEY AUTO_INCREMENT,
  rotation_id INT NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  hospital_id INT NOT NULL,
  discipline_id INT NOT NULL,
  FOREIGN KEY (hospital_id) REFERENCES tblHospital(id),
  FOREIGN KEY (discipline_id) REFERENCES tblDiscipline(id)
);

CREATE TABLE tblAssignedRotations (
  id INT PRIMARY KEY AUTO_INCREMENT,
  rotation_id INT NOT NULL,
  account_id INT NOT NULL,
  FOREIGN KEY (rotation_id) REFERENCES tblRotation(id),
  FOREIGN KEY (account_id) REFERENCES tblAccount(id)
);
