use criterion::{criterion_group, criterion_main, Criterion};
use twilight_http::client::Client;
use twilight_model::{
    application::command::permissions::{CommandPermissions, CommandPermissionsType},
    id::{marker::CommandMarker, Id},
};

fn commands(commands: usize, permissions: usize) -> Vec<(Id<CommandMarker>, CommandPermissions)> {
    (0..commands)
        .flat_map(|id| {
            (0..permissions).map(move |_| {
                (
                    Id::new(id as u64),
                    CommandPermissions {
                        id: CommandPermissionsType::Role(Id::new(4)),
                        permission: true,
                    },
                )
            })
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let client = Client::new(String::new());
    let application_id = Id::new(1);

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
                        .set_command_permissions(Id::new(2), &list)
                        .is_ok());
                });
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
