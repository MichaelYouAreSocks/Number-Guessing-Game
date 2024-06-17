pub fn score_board(high_scores: &Vec<String>) -> String {
    let layout: String = format!(
        "{}\n{}\n\n{}\n\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}",
        "-This file contains the high-score board for the Number Guessing Game.",
        "--------------------------------------------------------------",
        "\tRank\t\tSearch\tAtempts\tName",
        "\t1st\t:\t",
        high_scores[0],
        "\t2nd\t:\t",
        high_scores[1],
        "\t3rd\t:\t",
        high_scores[2],
        "\t4th\t:\t",
        high_scores[3],
        "\t5th\t:\t",
        high_scores[4],
        "\t6th\t:\t",
        high_scores[5],
        "\t7th\t:\t",
        high_scores[6],
        "\t8th\t:\t",
        high_scores[7],
        "\t9th\t:\t",
        high_scores[8],
        "\t10th\t:\t",
        high_scores[9],
    );
    return layout;
}
