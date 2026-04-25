#ifndef __CFGPARSER_H
#define __CFGPARSER_H

/*
enum mimicing the EncryptionType enum defined in the rust module.
*/
enum EncyptionType
{
    Xor,
    Viginere,
    Aes,
};

#ifdef __cplusplus
extern "C"
{
#endif

    /*
    function designed to read the end of the current binary and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are XOR-decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    note: if the key passed in is NULL, a default of 'q' will be used
    when attempting to decrypt the bytes.
    */
    extern char *read_cfg(char *key);

    /*
    function designed to read the end of the current binary and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are XOR-decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    the offset argument specifies how far back fron the end of the file
    the last byte of the size block will be found.

    note: if the key passed in is NULL, a default of 'q' will be used
    when attempting to decrypt the bytes.
    */
    extern char *read_cfg_o(char *key, unsigned int offset);

    /*
    function designed to read the end of the current binary and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    the type of encryption is specified by the "enc_type" argument.
    */
    extern char *read_cfg_with_encryption(char *key, int enc_type);

    /*
    function designed to read the end of the current binary and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    the type of encryption is specified by the "enc_type" argument.

    the offset argument specifies how far back fron the end of the file
    the last byte of the size block will be found.
    */
    extern char *read_cfg_with_encryption_o(char *key, int enc_type, unsigned int offset);

    /*
    function designed to read the end of a target file and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are XOR-decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    note: if the key passed in is NULL, a default of 'q' will be used
    when attempting to decrypt the bytes.
    */
    extern char *read_cfg_from_file(char *filename, char *key);

    /*
    function designed to read the end of a target file and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    the type of encryption is specified by the "enc_type" argument.
    */
    extern char *read_cfg_from_file_with_encryption(char *filename, char *key, int enc_type);

    /*
    function designed to read the end of a target file and
    extract configuration information from it. this will return
    the C2 address in the form "<scheme>://<ip>:<port>".

    the extracted bytes are decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    the type of encryption is specified by the "enc_type" argument.

    the offset specifies how far back to search from the end of the file
    to find the end of the size block of the configuration.
    */
    extern char *read_cfg_from_file_with_encryption_o(char *filename, char *key, int enc_type, unsigned int offset);

    /*
    function designed to free memory that was originally allocated
    by rust. since the pointer was created by the rust library, the
    rust library still owns that memory and, therefore, must be freed
    by the library.
    */
    extern void free_memory(char *ptr);

#ifdef __cplusplus
}
#endif

#endif // __CFGPARSER_H
