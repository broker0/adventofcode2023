
const RED_CUBES_MAX: usize = 12;
const GREEN_CUBES_MAX: usize = 13;
const BLUE_CUBES_MAX: usize = 14;

type CubeSet = (usize, usize, usize);  // R G B cubes count
type Game = (usize, Vec<CubeSet>);
type Games = Vec<Game>;

fn parse_input(input: &String) -> Games {
    let mut result = Games::new();

    for line in input.lines() {
        let game = line.split(": ").collect::<Vec<&str>>();
        let game_info = game[0].split(" ").collect::<Vec<&str>>();
        let game_id = game_info[1].parse::<usize>().unwrap();
        let game_cubes = game[1];

        let cubes_sets = game_cubes.split("; ").collect::<Vec<&str>>();

        let mut current_game = Vec::<CubeSet>::new();

        for set in cubes_sets {
            let cubes = set.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let cube_info = cube.split(" ").collect::<Vec<&str>>();
                let cube_count = cube_info[0].parse::<usize>().unwrap();
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                match cube_info[1] {
                    "red" => { red = cube_count },
                    "green" => { green = cube_count; },
                    "blue" => { blue = cube_count; },

                    _ => {
                        panic!("Invalid color {}", cube_info[1]);
                    }
                }

                current_game.push((red, green, blue));
            }
        }

        result.push((game_id, current_game));
    }

    result
}

pub fn stage1(input: &String) -> String {
    let games = parse_input(input);

    let sum_of_ids = games.iter().fold(
        0,
        |sum, game| {
            let (game_id, cube_sets) = game;
            for &(red,green, blue) in cube_sets {
                if red > RED_CUBES_MAX || green > GREEN_CUBES_MAX || blue > BLUE_CUBES_MAX {
                    return sum
                }
            }

            return sum + *game_id
        }
    );

    sum_of_ids.to_string()
}


pub fn stage2(input: &String) -> String {
    let games = parse_input(input);

    let sum_of_powers = games.iter().fold(
        0, |acc, game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let (_game_id, cube_sets) = game;
            for (red_count, green_count, blue_count) in cube_sets {
                red = red.max(*red_count);
                green = green.max(*green_count);
                blue = blue.max(*blue_count);
            }

            acc + (red*green*blue)
    });

    sum_of_powers.to_string()
}
