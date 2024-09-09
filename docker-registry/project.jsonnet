{
    targets: {
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
                            '--remove-orphans',
                            '--pull',
                            'always',
                            '--force-recreate'
                        ]
                    }
                ],
            }
        },
        'collect-garbage': {
            executor: 'std:commands',
            options: {
                commands: [
                    {
                        program: 'docker',
                        arguments: [
                            'compose', 
                            'exec', 
                            'registry', 
                            'bin/registry', 
                            'garbage-collect',
                            '/etc/docker/registry/config.yml'
                        ]
                    }
                ]
            }
        }
    }
}