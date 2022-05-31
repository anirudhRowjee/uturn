-- this is the SQL to setup a database from scratch for UTURN
-- Assumes you're logged in as root on a completely blank database.

-- Create the database
CREATE DATABASE UTURN;

-- Create the user who's going to access that database
CREATE USER 'uturn'@'localhost' identified by 'uturn_dbmanager';

-- Grant privileges to this user
GRANT ALL PRIVILEGES ON UTURN.* to 'uturn'@'localhost' WITH GRANT OPTION;
