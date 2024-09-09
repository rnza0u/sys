{
  targets: {
    'source-bin': {
      cache: {
        invalidateWhen: {
          inputChanges: [
            'Cargo.toml',
            'Cargo.lock',
            'src/**',
          ],
        },
      },
    },
    'build-bin': {
      executor: 'std:commands',
      description: 'Build CI binaries.',
      cache: {
        invalidateWhen: {
          outputChanges: [
            'target/x86_64-unknown-linux-musl/release/create-cache',
            'target/x86_64-unknown-linux-musl/release/restore-cache',
          ],
        },
      },
      options: {
        commands: [
          {
            program: 'cross',
            arguments: ['build', '--target', 'x86_64-unknown-linux-musl', '--release'],
          },
        ],
      },
      dependencies: [
        'source-bin',
      ],
    },
    build: {
        executor: 'std:commands',
        cache: {
            invalidateWhen: {
                inputChanges: [
                    'Dockerfile',
                    'conf/**'
                ]
            }
        },
        options: {
            commands: [
                {
                    program: 'docker',
                    arguments: [
                        'build',
                        '-t',
                        'registry.rnzaou.me/ci:latest',
                        '.'
                    ]
                }
            ]
        },
        dependencies: ['build-bin']
    },
    'publish': {
        executor: 'std:commands',
        options: {
            commands: [
                {
                    program: 'docker',
                    arguments: [
                        'push',
                        'registry.rnzaou.me/ci:latest'
                    ]
                }
            ]
        },
        dependencies: ['registry-authenticate', 'build']
    },
    'registry-authenticate': {
        executor: {
            url: 'https://github.com/rnza0u/blaze-executors.git',
            kind: 'Node',
            path: 'docker-authenticate',
            format: 'Git'
        },
        options: {
            registry: 'registry.rnzaou.me'
        }
    }
  },
}
