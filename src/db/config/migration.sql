-- Create Databse
CREATE DATABASE Users;

-- Create user for the authentication service to use
CREATE USER authservice; 
GRANT CONNECT ON DATABASE Users TO authservice;

-- Another user for the notification service and the billing one (read only access)
CREATE USER notifservice;
GRANT CONNECT ON DATABASE Users TO notifservice;

CREATE USER billingservice;
GRANT CONNCET ON DATABASE Users TO billingservice;

-- TODO: Set the previous roles as read-only

-- Create basic information table
CREATE TABLE UserBasic (
    nickname varchar(127) NOT NULL PRIMARY KEY,
    email varchar(127) NOT NULL UNIQUE
);

-- Create Access information
CREATE TABLE UserLogin (
    email varchar(127),
    login_method varchar(127) NOT NULL CHECK (login_method IN('Google', 'Password')) DEFAULT 'Password',
    hash_pass varchar(255),
    CONSTRAINT fk_info
        FOREIGN KEY(email)
            REFERENCES UserBasic(email)
            ON DELETE SET NULL
);

-- User Information
CREATE TABLE UserInfo (
    email varchar(127),
    username varchar(127) NOT NULL,
    surname  varchar(127) NOT NULL,
    address  varchar(255),
    CONSTRAINT fk_info
        FOREIGN KEY(email)
            REFERENCES UserBasic(email)
            ON DELETE SET NULL
);

-- User metadata

CREATE TABLE UserMetaInfo (
    email varchar(127),
    create_at timestamp NOT NULL,
    expire_on timestamp,
    ip_from   INT NOT NULL,
    CONSTRAINT fk_info
        FOREIGN KEY(email)
            REFERENCES UserBasic(email)
            ON DELETE SET NULL
);
