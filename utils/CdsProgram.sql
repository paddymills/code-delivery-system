CREATE TABLE CdsProgram (
    ID int not null identity(1,1) primary key,
    Program varchar(255) not null,
    CheckedBy varchar(255),
    DatePrinted date,
    Comments varchar(255)
);
