use std::io::BufRead;
use std::io::{BufReader, Write};
use std::process::{Command, Stdio};

#[test]
fn test_deduplication() {
    // Run the dedupfeed program as a child process
    let mut child = Command::new("target/debug/dedupfeed")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start dedupfeed process");

    // Feed in sample Apache log file data to the program's stdin
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let file =
        std::fs::File::open("tests/sample_access.log").expect("Failed to open sample log file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        writeln!(stdin, "{}", line.unwrap()).expect("Failed to write to stdin");
    }

    // Collect the program's output from stdout
    let output = child
        .wait_with_output()
        .expect("Failed to read child process output");

    let expected_output = "\
        127.0.0.1 - - [23/Mar/2023:17:32:56 +0000] \"GET /index.html HTTP/1.1\" 200 101 \"-\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\"\n\
        127.0.0.1 - - [23/Mar/2023:17:33:12 +0000] \"GET /styles.css HTTP/1.1\" 200 48 \"http://localhost:8000/index.html\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\"\n\
        127.0.0.1 - - [23/Mar/2023:17:33:22 +0000] \"GET /scripts.js HTTP/1.1\" 200 38 \"http://localhost:8000/index.html\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\"\n\
        127.0.0.1 - - [23/Mar/2023:17:33:22 +0000] \"GET /scripts.js HTTP/1.1\" 200 38 \"http://localhost:8000/index.html\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\" (seen in the last 5 seconds)\n\
        127.0.0.1 - - [19/Dec/2020:18:18:46 +0100] \"GET /index.php?option=com_contact&view=contact&id=1 HTTP/1.1\" 200 9873 \"-\" \"Mozilla/5.0(WindowsNT10.0;WOW64)AppleWebKit/537.36(KHTML,likeGecko)Chrome/63.0.3235.0Safari/537.36\" \"-\"\n\
        127.0.0.1 - - [19/Dec/2020:18:18:46 +0100] \"GET /index.php?option=com_contact&view=contact&id=1 HTTP/1.1\" 200 9873 \"-\" \"Mozilla/5.0(WindowsNT10.0;WOW64)AppleWebKit/537.36(KHTML,likeGecko)Chrome/63.0.3235.0Safari/537.36\" \"-\" (seen in the last 5 seconds)\n\
        127.0.0.1 - - [23/Mar/2023:17:33:31 +0000] \"GET /images/logo.png HTTP/1.1\" 200 7198 \"http://localhost:8000/index.html\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\"\n\
        127.0.0.1 - - [23/Mar/2023:17:34:01 +0000] \"GET /favicon.ico HTTP/1.1\" 404 481 \"-\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\"\n\
        127.0.0.1 - - [23/Mar/2023:17:34:01 +0000] \"GET /favicon.ico HTTP/1.1\" 404 481 \"-\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:97.0) Gecko/20100101 Firefox/97.0\" (seen in the last 5 seconds)\n\
        127.0.0.1 - - [19/Dec/2020:18:18:47 +0100] \"POST /index.php?option=com_contact&view=contact&id=1 HTTP/1.1\" 200 188 \"-\" \"Mozilla/5.0(WindowsNT10.0;WOW64)AppleWebKit/537.36(KHTML,likeGecko)Chrome/63.0.3235.0Safari/537.36\" \"-\"\n";
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_output);
}
