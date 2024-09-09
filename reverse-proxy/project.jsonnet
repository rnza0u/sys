local image = 'registry.rnzaou.me/reverse-proxy';

{
    targets: {
        'docker-authenticate': {
            executor: {
                url: 'https://github.com/rnza0u/blaze-executors.git',
                path: 'docker-authenticate',
                format: 'Git'
            }
        },
        build: {
            executor: 'std:commands',
            options: {
                commands: [
                    {
                        program: 'docker',
                        arguments: [
                            'build',
                            '-t',
                            image,
                            '.'
                        ]
                    }
                ]
            },
            cache: {
                invalidateWhen: {
                    inputChanges: [
                        'Dockerfile',
                        'nginx.conf',
                        'conf.d/**'
                    ]
                }
            }
        },
        push: {
            executor: 'std:commands',
            options: {
                commands: [
                    {
                        program: 'docker',
                        arguments: [
                            'push',
                            image
                        ]
                    }
                ]
            },
            dependencies: [
                'docker-authenticate',
                'build'
            ]
        },
        'renew-certificates': {
            executor: 'std:commands',
            options: {
                commands: [
                    {
                        program: 'docker',
                        arguments: [
                            'compose',
                            'exec',
                            '-t',
                            'reverse-proxy',
                            '/scripts/run_certbot.sh',
                            'force'
                        ]
                    }
                ]
            }
        },
        deploy: {
            executor: 'std:commands',
            options: {
                commands: [
                    {
                        program: 'docker',
                        arguments: [
                            'compose',
                            'up',
                            '--detach',
                            '--pull',
                            'always',
                            '--force-recreate',
                            '--remove-orphans'
                        ]
                    }
                ]
            }
        }
    }
}