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
