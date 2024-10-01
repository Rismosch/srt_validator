use std::io::Read;
use std::path::PathBuf;

fn main() {
    let src = PathBuf::from("insert path here");

    let mut file = std::fs::File::open(src).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let content_lines = content.lines().collect::<Vec<_>>();

    let mut content_line_num = 0;
    let mut subtitle_num = 0;

    let mut previous_id = -1;
    let mut existing_ids = std::collections::HashSet::new();

    while content_line_num < content_lines.len() {

        let mut subtitle_lines = Vec::new();

        loop {
            let Some(content_line) = content_lines.get(content_line_num) else {
                break;
            };

            content_line_num += 1;
            subtitle_lines.push(*content_line);

            if content_line.is_empty() {
                break;
            }
        }

        assert!(subtitle_lines.len() > 2, "{:?}", subtitle_lines);

        let id = subtitle_lines[0];
        let time_stamp = subtitle_lines[1];
        let subtitle = subtitle_lines[2..].join("\n");

        println!(
            "subtitle: {:?}, id: {:?}, time: {:?}, subtitle: {:?}",
            subtitle_num,
            id,
            time_stamp,
            subtitle,
        );

        // attempt to parse id, check if it increases and if it is unique
        let parsed_id = id.parse::<isize>().unwrap();
        assert!(parsed_id > previous_id);
        previous_id = parsed_id;
        assert!(existing_ids.insert(parsed_id));

        // validate timestamp
        let splits = time_stamp.split(" --> ").collect::<Vec<_>>();
        assert!(splits.len() == 2);
        let left = to_ms(splits[0]);
        let right = to_ms(splits[1]);
        assert!(left < right);

        subtitle_num += 1;
    }
}

fn to_ms(value: &str) -> usize {
    let splits = value.split(",").collect::<Vec<_>>();
    assert!(splits.len() == 2);
    let hhmmss = splits[0];
    let ms = splits[1];

    let splits = hhmmss.split(":").collect::<Vec<_>>();
    assert!(splits.len() == 3);
    let h = splits[0];
    let m = splits[1];
    let s = splits[2];

    assert!(h.len() == 2);
    assert!(m.len() == 2);
    assert!(s.len() == 2);
    assert!(ms.len() == 3);

    let hours = h.parse::<usize>().unwrap();
    let minutes = m.parse::<usize>().unwrap();
    let seconds = s.parse::<usize>().unwrap();
    let millis = ms.parse::<usize>().unwrap();


    let s_millis = seconds * 1000;
    let m_millis = minutes * 1000 * 60;
    let h_millis = hours * 1000 * 60 * 60;

    millis + s_millis + m_millis + h_millis
}
