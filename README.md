# RustyLnk
LNK to JSON

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
              "long_string_size": 0,
              "name": "Alloy Research",
              "long_name": null,
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
              "long_string_size": 0,
              "name": "Detailed Documents",
              "long_name": null,
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
              "long_string_size": 0,
              "name": "Copy of Metal Alloy List Research.xlsx",
              "long_name": null,
              "version_offset": 28
            }
          }
        }
      }
    }]
  },
  "location_info": {
    "info_size": 166,
    "header_size": 28,
    "flags": 1,
    "offset_vol_info": 28,
    "offset_loc_path": 45,
    "offset_net_share": 0,
    "offset_common_path": 165,
    "volume_info": {
      "vi_size": 17,
      "drive_type": 3,
      "serial_number": 538397730,
      "offset_vol_label": 16,
      "offset_vol_label_unicode": null,
      "volume_label": "",
      "volume_label_unicode": null
    },
    "local_path": "C:\\Documents and Settings\\tdungan\\My Documents\\Alloy Research\\Detailed Documents\\Copy of Metal Alloy List Research.xlsx",
    "netshare_info": {
      "size": 166,
      "flags": 28,
      "offset_share_name": 1,
      "offset_device_name": 28,
      "provider_type": 45,
      "offset_share_name_unicode": null,
      "offset_device_name_unicode": null,
      "share_name": "",
      "device_name": "\u0011",
      "share_name_unicode": null,
      "device_name_unicode": null
    },
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
      "size": 88,
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
