# FireFile RFMP server and client 🔥📒

FireFile is an implementation of both an RFMP server (Rust) and client (Python).

Remote Filesystem Management Protocol (RFMP)
==========

RFMP is a filesystem management protocol that allows to manage the metadata and general top-level filesystem operations for filesystem objects (files and directories) on remote servers, on top of REST (HTTP).

It provides a set of protocol actions to be used by clients:

* **FCREATE:** Creates a file in the server.
* **DCREATE:** Creates a directory in the server.
* **EXISTS:** Checks if a filesystem object exists in the server.
* **METADATA:** Retrieves the metadata of a filesystem object.
* **DELETE:** Deletes a filesystem object from the server.
* **TRANSFER:** Moves a filesystem object from one directory to another.
* **RENAME:** Renames a filesystem object in the server.
* **COPY:** Copies a filesystem object to another directory.