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
                            '-d'
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