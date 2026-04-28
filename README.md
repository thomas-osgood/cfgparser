# CFGParser

## Overview

The purpose of this repository is to have a library that can read configuration data embedded in a file.

The configuration is expected to be embedded in an obfuscated format in the file. The configuration format is a JSON marshalled, base64-encoded, then encrypted byte package. To determine the size of the encrypted configruation, an 8 byte block is expected to be found immediately after the configuration bytes.

The valid encryption types are defined in `cfgparser_encryption`'s `EncryptionType` enum.

**_important note: this library does not handle the embedding of the configuration, only the extraction._**

### Embedding Process

This section describes the expected embedding process this library will reverse to extract the configuration.

![encoding process](./media/encoding_process.jpg)

The encrypted output is then expected to be appended to the end of the original binary along with an 8 byte block holding the size of the encrypted packet.

![append overview](./media/append_process.jpg)

This library reversed the process described/illustrated above to transform the configuration bytes into a configuration structure.

### Extract Process

![extract process](./media/extract_process.jpg)
