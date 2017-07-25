# RustyLnk
A fast and cross platform LNK Parser written in Rust that gives you the ability to query the records via JMESPath queries. Output is JSONL.

```
RusyLnk 0.1.0
Matthew Seyer <https://github.com/forensicmatt/RustyLnk>
LNK Parser written in Rust.

USAGE:
    RustyLnk.exe [FLAGS] [OPTIONS]

FLAGS:
    -b, --bool_expr    JMES Query as bool only. (Prints whole record if true.)
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -q, --query <QUERY>    JMES Query
    -s, --source <PATH>    The LNK file or folder with LNK files to parse.

```

### Example Output
```json
{
  "header": {
    "header_size": 76,
    "guid": "00021401-0000-0000-C000-000000000046",
    "data_flags": "HAS_TARGET_ID_LIST | HAS_LINK_INFO | HAS_RELATIVE_PATH | HAS_WORKING_DIR | IS_UNICODE",
    "file_flags": "FILE_ATTRIBUTE_ARCHIVE",
    "created": "2012-03-08 22:11:26.372",
    "accessed": "2012-03-16 20:03:34.936",
    "modified": "2012-03-08 22:11:26.841",
    "file_size": 68346,
    "icon_offset": 0,
    "window_flag": 1,
    "hot_key": 0
  },
  "target_list": {
    "list_size": 310,
    "shell_items": [{
      "data": {
        "class_type": "0x1F",
        "unknown": 72,
        "content": "BA8F0D4525ADD01198A80800361B1103"
      }
    },
    {
      "data": {
        "class_type": "0x31",
        "unknown": 0,
        "content": {
          "sub_flags": "DIRECTORY",
          "file_size": 0,
          "last_modification": "2012-03-12 21:27:04.000",
          "flags": "FILE_ATTRIBUTE_DIRECTORY",
          "name": "ALLOYR~1",
          "extention_block": {
            "header": {
              "version": 3,
              "signature": "0xBEEF0004"
            },
            "content": {
              "creation": "2012-03-09 17:41:52.000",
              "last_access": "2012-03-16 20:03:22.000",
              "identifier": 20,
              "name": "Alloy Research",
              "version_offset": 24
            }
          }
        }
      }
    },
    {
      "data": {
        "class_type": "0x31",
        "unknown": 0,
        "content": {
          "sub_flags": "DIRECTORY",
          "file_size": 0,
          "last_modification": "2012-03-16 20:03:36.000",
          "flags": "FILE_ATTRIBUTE_DIRECTORY",
          "name": "DETAIL~1",
          "extention_block": {
            "header": {
              "version": 3,
              "signature": "0xBEEF0004"
            },
            "content": {
              "creation": "2012-03-08 22:10:52.000",
              "last_access": "2012-03-16 20:03:36.000",
              "identifier": 20,
              "name": "Detailed Documents",
              "version_offset": 24
            }
          }
        }
      }
    },
    {
      "data": {
        "class_type": "0x32",
        "unknown": 0,
        "content": {
          "sub_flags": "FILE",
          "file_size": 68346,
          "last_modification": "2012-03-08 22:11:28.000",
          "flags": "FILE_ATTRIBUTE_ARCHIVE",
          "name": "COPYOF~1.XLS",
          "extention_block": {
            "header": {
              "version": 3,
              "signature": "0xBEEF0004"
            },
            "content": {
              "creation": "2012-03-08 22:11:28.000",
              "last_access": "2012-03-16 20:03:32.000",
              "identifier": 20,
              "name": "Copy of Metal Alloy List Research.xlsx",
              "version_offset": 28
            }
          }
        }
      }
    }]
  },
  "location_info": {
    "flags": "VolumeIDAndLocalBasePath",
    "volume_info": {
      "drive_type": "DRIVE_FIXED",
      "serial_number": "2017-4C22",
      "volume_label": ""
    },
    "local_path": "C:\\Documents and Settings\\tdungan\\My Documents\\Alloy Research\\Detailed Documents\\Copy of Metal Alloy List Research.xlsx",
    "common_path": ""
  },
  "data_strings": {
    "description": null,
    "relative_path": "..\\My Documents\\Alloy Research\\Detailed Documents\\Copy of Metal Alloy List Research.xlsx",
    "working_directory": "C:\\Documents and Settings\\tdungan\\My Documents\\Alloy Research\\Detailed Documents",
    "command_line_args": null,
    "icon_location": null
  },
  "extra_data": {
    "distributed_tracker": {
      "version": 0,
      "machine_id": "wks-winxp32bit",
      "droid_volume": "79CBEB4E-F29D-4A0C-A70E-E5647A53970B",
      "droid_file": "D919757C-66D3-11E1-A3F6-005056A50010",
      "birth_droid_volume": "79CBEB4E-F29D-4A0C-A70E-E5647A53970B",
      "birth_droid_file": "D919757C-66D3-11E1-A3F6-005056A50010"
    },
    "special_folder": {
      "special_folder_id": 5,
      "first_child_segment_offset": 20
    }
  }
}
```
