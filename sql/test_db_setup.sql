DROP DATABASE IF EXISTS ApolloniusTesting;
CREATE DATABASE ApolloniusTesting;
USE ApolloniusTesting;

DROP TABLE IF EXISTS tblAccount ;
DROP TABLE IF EXISTS tblHospital ;
DROP TABLE IF EXISTS tblDiscipline ;
DROP TABLE IF EXISTS tblSessionToken ;
DROP TABLE IF EXISTS tblNotes ;
DROP TABLE IF EXISTS tblProtocol ;
DROP TABLE IF EXISTS tblEvents ;
DROP TABLE IF EXISTS tblRotation ;
DROP TABLE IF EXISTS tblSecurityQuestions ;
DROP TABLE IF EXISTS tblSecurityAnswers ;
DROP TABLE IF EXISTS tblStaff ;

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
  hospital_name VARCHAR(255) NOT NULL
);

CREATE TABLE tblDiscipline (
  discipline_id INT PRIMARY KEY AUTO_INCREMENT,
  discipline_name VARCHAR(255) NOT NULL
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
  content TEXT NOT NULL,
  public BOOLEAN NOT NULL DEFAULT 0,
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblProtocol (
  protocol_id INT PRIMARY KEY AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  content TEXT
);

CREATE TABLE tblEvents (
  event_id INT PRIMARY KEY AUTO_INCREMENT,
  account_id INT NOT NULL,
  start_date DATETIME NOT NULL,
  end_date DATETIME NOT NULL,
  event_name VARCHAR(255) NOT NULL,
  description TEXT,
  FOREIGN KEY (account_id) REFERENCES tblAccount(account_id)
);

CREATE TABLE tblRotation (
  rotation_id INT PRIMARY KEY AUTO_INCREMENT,
  event_id INT NOT NULL,
  hospital_id INT NOT NULL,
  discipline_id INT NOT NULL,
  FOREIGN KEY (event_id) REFERENCES tblEvents(event_id),
  FOREIGN KEY (hospital_id) REFERENCES tblHospital(hospital_id),
  FOREIGN KEY (discipline_id) REFERENCES tblDiscipline(discipline_id)
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

CREATE TABLE tblStaff (
  staff_id INT PRIMARY KEY AUTO_INCREMENT,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  cell_number VARCHAR(255) NOT NULL
);

INSERT INTO tblAccount (email, hashed_password, username, cell_number, profile_photo) VALUES 
('0000000@students.wits.ac.za', '0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e', 'test_account_has_everything', '1234567890', '0x0123456789ABCDEF'), /* password_1 */
('1111111@students.wits.ac.za', '0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e', 'test_account_missing_everything', '1234567890', '0x0123456789ABCDEF'); /* password_1 */

INSERT INTO tblSecurityQuestions (question) VALUES
('What was the name of your first pet?'),
('What is the name of your favourite sports team?');

INSERT INTO tblSecurityAnswers (secques_id, account_id, answer) VALUES
(1, 1, 'a459891617d735655dcfed3e37db66fa07f0175866ebf35f9de8ccc59c0840bb'), /*jeffrey*/
(2, 1, 'b93b9776163702f1fad6cbaf815326a41b3285d0961f4e838ebdb8ad52e5f16e'); /*manchester united*/

INSERT INTO tblProtocol (title, content) VALUES
('HHHH', 'Hazard - Identify any hazards that can cause harm to both you and the casulty.\nHello - Introduce yourself yourself to the casualty, and ask for consent to administer treatment.\nHistory - Ask the casualty what happened.\nHelp - call emergency services if necessary, else administer first aid.'),
('ABC', 'Airway - Check that the casualtys airway is unblocked.\nBreathing - Check that the casualty is breathing.\nCirculation - Check that the casulty has circulation with a capillary pinch test, or by measuring their pulse.\nIf any of these fail, administer CPR immediately'),
('CPR', '1. Lean the casultys head back, with their mouth open.\n2. Perform 30 chest compressions at 100 beats per minute, or to the beat of Stayin Alive by the Bee Gees on the casultys solar plexus / end of their sternum.\n3. After 30 compressions, administer 2 breaths to the casulty.\n4.Repeat until emergency services arrive!');

INSERT INTO tblNotes (account_id, title, content, public) VALUES
(1, 'Lecture Notes', 'Today we covered respiratory physiology. Here are the key points:\n\n* Oxygen and carbon dioxide exchange occurs in the alveoli of the lungs.\n* The respiratory system is controlled by the medulla oblongata in the brainstem.\n* The diaphragm and intercostal muscles are responsible for breathing.\n\n---\n\n', 0);

INSERT INTO tblHospital (hospital_name) VALUES
('St. Marys Hospital');

INSERT INTO tblDiscipline (discipline_name) VALUES
('Cardiology');

INSERT INTO tblEvents (account_id, start_date, end_date, event_name, description) VALUES 
(1, '2022-01-01', '2022-01-07', 'New Year Event', 'Celebrating the New Year'),
(1, '2022-02-01', '2022-02-20', 'Rotation 1', 'First rotation of the year');


INSERT INTO tblRotation (event_id, hospital_id, discipline_id) VALUES 
(2, 1, 1);

INSERT INTO tblStaff (first_name, last_name, email, cell_number) VALUES
('John', 'Doe', 'john.doe@wits.ac.za', '123-456-7890'),
('Jane', 'Smith', 'jane.smith@wits.ac.za', '987-654-3210'),
('David', 'Johnson', 'david.johnson@wits.ac.za', '555-555-5555'),
('Sarah', 'Williams', 'sarah.williams@wits.ac.za', '999-999-9999');