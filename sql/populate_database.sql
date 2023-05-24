INSERT INTO tblAccount (email, hashed_password, username, cell_number, profile_photo) 
VALUES 
('2763528@students.wits.ac.za', '0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e', 'user1', '1234567890', '0x0123456789ABCDEF'),
('4759170@students.wits.ac.za', '6cf615d5bcaac778352a8f1f3360d23f02f34ec182e259897fd6ce485d7870d4', 'user2', '2345678901', '0xFEDCBA9876543210'),
('9837462@students.wits.ac.za', '5906ac361a137e2d286465cd6588ebb5ac3f5ae955001100bc41577c3d751764', 'user3', '3456789012', '0xABCDEF0123456789'),
('6283928@students.wits.ac.za', 'b97873a40f73abedd8d685a7cd5e5f85e4a9cfb83eac26886640a0813850122b', 'user4', '4567890123', '0x0123456789ABCDEF'),
('1827364@students.wits.ac.za', '8b2c86ea9cf2ea4eb517fd1e06b74f399e7fec0fef92e3b482a6cf2e2b092023', 'user5', '5678901234', '0xFEDCBA9876543210'),
('9473628@students.wits.ac.za', '598a1a400c1dfdf36974e69d7e1bc98593f2e15015eed8e9b7e47a83b31693d5', 'user6', '6789012345', '0xABCDEF0123456789'),
('3829104@students.wits.ac.za', '5860836e8f13fc9837539a597d4086bfc0299e54ad92148d54538b5c3feefb7c', 'user7', '7890123456', '0x0123456789ABCDEF'),
('7462839@students.wits.ac.za', '57f3ebab63f156fd8f776ba645a55d96360a15eeffc8b0e4afe4c05fa88219aa', 'user8', '8901234567', '0xFEDCBA9876543210'),
('1029384@students.wits.ac.za', '9323dd6786ebcbf3ac87357cc78ba1abfda6cf5e55cd01097b90d4a286cac90e', 'user9', '9012345678', '0xABCDEF0123456789'),
('9203847@students.wits.ac.za', 'aa4a9ea03fcac15b5fc63c949ac34e7b0fd17906716ac3b8e58c599cdc5a52f0', 'user10', '0123456789', '0x0123456789ABCDEF');

INSERT INTO tblSecurityQuestions (question) VALUES
('What was the name of your first pet?'),
('What is the name of your favourite sports team?'),
('What is your astrological sign?'),
('What did you do for your first birthday?'),
('What high school did you go to?');

INSERT INTO tblSecurityAnswers (secques_id, account_id, answer) VALUES
(1, 1, 'a459891617d735655dcfed3e37db66fa07f0175866ebf35f9de8ccc59c0840bb'), /*jeffrey*/
(2, 1, 'b93b9776163702f1fad6cbaf815326a41b3285d0961f4e838ebdb8ad52e5f16e'), /*manchester united*/
(3, 2, 'c34ce311989d8bf0a65b9272905811c857e8c975313327c3deca09d3f95409e1'), /*taurus*/
(4, 2, '1584ecc2592eccae2314eaa81e61cf4ab9674f7508d77c3bd8acebff52acdb53'), /*play videogames*/
(5, 3, '0b4f17f89076d53c785b72e1ac4c0da8abc1e33ec6215e6806efc1f97696ce09'), /*edenvale high school*/
(1, 3, '730551f5bad4af0604f661e7f8b82e6a6c364ca19ce140166333d86cdc814ca6'); /*snowball*/

INSERT INTO tblProtocol (title, content) VALUES
('HHHH', 'Hazard - Identify any hazards that can cause harm to both you and the casulty.\nHello - Introduce yourself yourself to the casualty, and ask for consent to administer treatment.\nHistory - Ask the casualty what happened.\nHelp - call emergency services if necessary, else administer first aid.'),
('ABC', 'Airway - Check that the casualtys airway is unblocked.\nBreathing - Check that the casualty is breathing.\nCirculation - Check that the casulty has circulation with a capillary pinch test, or by measuring their pulse.\nIf any of these fail, administer CPR immediately'),
('CPR', '1. Lean the casultys head back, with their mouth open.\n2. Perform 30 chest compressions at 100 beats per minute, or to the beat of Stayin Alive by the Bee Gees on the casultys solar plexus / end of their sternum.\n3. After 30 compressions, administer 2 breaths to the casulty.\n4.Repeat until emergency services arrive!');

INSERT INTO tblNotes (account_id, title, content) VALUES
(1, 'Lecture Notes', 'Today we covered respiratory physiology. Here are the key points:\n\n* Oxygen and carbon dioxide exchange occurs in the alveoli of the lungs.\n* The respiratory system is controlled by the medulla oblongata in the brainstem.\n* The diaphragm and intercostal muscles are responsible for breathing.\n\n---\n\n'),
(2, 'Study Group', 'Meeting with classmates to review material for upcoming exam. We covered:\n\n* Endocrine system\n* Renal system\n* Hematology\n\nWe created flashcards to help us memorize key concepts.\n\n---\n\n'),
(3, 'Patient Rounds', 'Observed a patient with COPD this morning. Here are my observations:\n\n* The patient had difficulty breathing and used a nebulizer to help open their airways.\n* The patient was on supplemental oxygen via nasal cannula.\n* We discussed the importance of smoking cessation and medication adherence with the patient.\n\n---\n\n'),
(4, 'Medical Ethics', 'Discussed ethical dilemmas in medicine during seminar. We covered:\n\n* Patient autonomy\n* Informed consent\n* End-of-life care\n\nWe debated various scenarios and discussed how to handle ethical dilemmas in practice.\n\n---\n\n'),
(5, 'Clinical Rotation', 'Started my rotation in the cardiology ward. Here are some interesting cases:\n\n* Patient with atrial fibrillation and heart failure\n* Patient with myocardial infarction\n* Patient with hypertrophic cardiomyopathy\n\nI observed procedures such as angiograms and echocardiograms.\n\n---\n\n'),
(6, 'Research Paper', 'Working on a paper about the effects of exercise on heart health. My research has shown that:\n\n* Regular exercise can reduce the risk of heart disease.\n* Exercise can improve cardiovascular function and lower blood pressure.\n* Exercise can also have psychological benefits, such as reducing stress and anxiety.\n\n---\n\n'),
(7, 'Grand Rounds', 'Attended a presentation on rare diseases. Some of the diseases discussed were:\n\n* Marfan syndrome\n* Huntingtons disease\n* Alpha-1 antitrypsin deficiency\n\nWe also discussed the challenges of diagnosing and treating rare diseases.\n\n---\n\n'),
(8, 'Anatomy Lab', 'Dissected a cadaver to study the musculoskeletal system. We identified and studied the following structures:\n\n* Bones: femur, tibia, fibula, humerus, radius, ulna\n* Joints: hip, knee, ankle, shoulder, elbow, wrist\n* Muscles: quadriceps, hamstrings, biceps, triceps, deltoids\n\n---\n\n'),
(9, 'Emergency Room', 'Worked in the ER and saw several cases of trauma. Some of the cases were:\n\n* Motor vehicle accident\n* Gunshot wound\n* Fall from height\n\nI assisted with procedures such as intubation, chest tube placement, and wound care.\n\n---\n\n'),
(10, 'Surgery Rotation', 'Started my rotation in the surgical ward. Observed several procedures, including:\n\n* Appendectomy\n* Cholecystectomy\n* Hysterectomy\n\nI also learned about postoperative care and wound management.\n\n---\n\n');

INSERT INTO tblHospital (hospital_name) VALUES
('St. Marys Hospital'),
('Johns Hopkins Hospital'),
('Cedars-Sinai Medical Center'),
('Mayo Clinic'),
('Massachusetts General Hospital');

INSERT INTO tblDiscipline (discipline_name)
VALUES
('Cardiology'),
('Dermatology'),
('Endocrinology'),
('Gastroenterology'),
('Hematology');

INSERT INTO tblEvents (account_id, start_date, end_date, event_name, description) VALUES 
(1, '2022-01-01', '2022-01-07', 'New Year Event', 'Celebrating the New Year'),
(1, '2022-02-14', '2022-02-14', 'Valentines Day Event', 'Celebrate love on Valentines Day'),
(2, '2022-07-04', '2022-07-04', 'Independence Day Event', 'Celebrate Americas Independence Day'),
(3, '2022-10-31', '2022-10-31', 'Halloween Event', 'Dress up and have fun on Halloween'),
(4, '2022-12-25', '2022-12-25', 'Christmas Event', 'Celebrate the birth of Jesus Christ');

INSERT INTO tblRotation (event_id, hospital_id, discipline_id)
VALUES 
  (1, 1, 1),
  (1, 2, 2),
  (2, 3, 5),
  (3, 2, 1),
  (4, 5, 4);

INSERT INTO tblStaff (first_name, last_name, email, cell_number) VALUES
('John', 'Doe', 'john.doe@wits.ac.za', '123-456-7890'),
('Jane', 'Smith', 'jane.smith@wits.ac.za', '987-654-3210'),
('David', 'Johnson', 'david.johnson@wits.ac.za', '555-555-5555'),
('Sarah', 'Williams', 'sarah.williams@wits.ac.za', '999-999-9999');