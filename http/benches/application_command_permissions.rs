use criterion::{criterion_group, criterion_main, Criterion};
use twilight_http::{client::Client, request::application::SetCommandPermissions};
use twilight_model::{
    application::command::permissions::{CommandPermissions, CommandPermissionsType},
    id::{ApplicationId, CommandId, GuildId, RoleId},
};

fn commands(commands: usize, permissions: usize) -> Vec<(CommandId, CommandPermissions)> {
    (0..commands)
        .map(|id| {
            (0..permissions).map(move |_| {
                (
                    CommandId(id as u64),
                    CommandPermissions {
                        id: CommandPermissionsType::Role(RoleId(4)),
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
    client.set_application_id(ApplicationId(1));

    let command_counts = [5usize, 10, 50, 100];
    let permission_counts = [2usize, 5, 10];

    for command in command_counts {
        for permission in permission_counts {
            let name = format!("{} commands, {} permissions", command, permission);

            c.bench_function(&name, |b| {
                let list = commands(command, permission);

                b.iter(|| {
                    assert!(client.set_command_permissions(GuildId(2), &list).is_ok());
                });
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
