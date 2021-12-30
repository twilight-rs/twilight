use criterion::{criterion_group, criterion_main, Criterion};
use twilight_http::client::Client;
use twilight_model::{
    application::command::permissions::{CommandPermissions, CommandPermissionsType},
    id::{marker::CommandMarker, Id},
};

fn commands(commands: usize, permissions: usize) -> Vec<(Id<CommandMarker>, CommandPermissions)> {
    (0..commands)
        .map(|id| {
            (0..permissions).map(move |_| {
                (
                    Id::new_checked(id as u64).expect("non zero"),
                    CommandPermissions {
                        id: CommandPermissionsType::Role(Id::new_checked(4).expect("non zero")),
                        permission: true,
                    },
                )
            })
        })
        .flatten()
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let client = Client::new(String::new());
    let application_id = Id::new_checked(1).expect("non zero");

    let command_counts = [5usize, 10, 50, 100];
    let permission_counts = [2usize, 5, 10];

    for command in command_counts {
        for permission in permission_counts {
            let name = format!("{} commands, {} permissions", command, permission);

            c.bench_function(&name, |b| {
                let list = commands(command, permission);

                b.iter(|| {
                    assert!(client
                        .interaction(application_id)
                        .set_command_permissions(Id::new_checked(2).expect("non zero"), &list)
                        .is_ok());
                });
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
