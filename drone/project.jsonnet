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
                            '--force-recreate',
                            '--pull',
                            'always',
                            '--detach',
                            '--remove-orphans'
                        ]
                    }
                ]
            }
        }
    }
}