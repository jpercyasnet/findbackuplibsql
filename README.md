# findbackuplibsql
Pure Rust program: Read the screened and formated list of windows files and see if they exist in the Backup Database. Output files reflecting the found and not found. This is used to restore files.

This program uses libsql vs findbackupdb uses sql3 interface

example:

findbackup01 bk20241103a.db3 20250217List.csv_out02.csv

bk20241103a.db3 is backup database 

20250217List.csv_out02.csv is the screen and formated list of windows files from windirparse

very slow because of database calls but 7% faster than findbackupdb

35% faster when copying database to ram:

sudo mkdir /tmp/ramdisk

sudo chmod 777 /tmp/ramdisk

sudo mount -t tmpfs -o size=2G myramdisk /tmp/ramdisk

sqlite3 bk20241103a.db3 ".backup '/tmp/ramdisk/backup.db3'"

findbackup01 /tmp/ramdisk/backup.db3 20250217List.csv_out02.csv
