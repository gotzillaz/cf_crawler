# cf_crawler
This is `cf_crawler` for crawling the `accepted` submissions from Codeforces.
The main purposes of this repository are to make an archive of my own Codeforeces submissions and to learn how to write Rust language.
Note that the coding style may not follow the best practice as I am still in the early learning stage. 

## Usage
To build the code:
``` bash
cargo build
```
To run the code:
``` bash
cargo run
```


## Environment file
There are four required parameters in the `.env` file. Please first check `.env.template`, then modify and rename it to `.env`.
- `SUBMISSION_JSON_PATH`: a path to a JSON array that describes the list of target submissions. This can be obtained by using Codeforces API ([link](https://codeforces.com/apiHelp/objects#Submission)).
- `LANG_EXT_PATH`: a path to a JSON object that maps the `programmingLanguage` to file extension.
Please check the example below.
```json
{
    "GNU C++20 (64)": ".cpp",
    "PyPy 3-64": ".py",
    "Java 7": ".java",
}
```
- `HTML_PATH`: a path for storing the downloaded submission html files. 
- `OUTPUT_PATH`: a path for storing the source codes, parsed from html files.