{
    # the workspace name
    name: 'sys',
    # add an entry to this dictionnary everytime you want to create a new project.
    projects: {
        'cross-builders': {
            path: 'cross-builders',
            description: 'Custom docker images for cross-rs'
        },
        drone: {
            path: 'drone',
            description: 'Drone CI server (drone.rnzaou.me)'
        },
        'reverse-proxy': {
            path: 'reverse-proxy',
            description: 'Nginx reverse proxy'
        },
        'docker-registry': {
            path: 'docker-registry',
            description: 'Self hosted docker registry (registry.rnzaou.me)'
        },
        'ci-image': {
            path: 'ci-image',
            description: 'Main CI docker image'
        }
    },
    # workspace global settings
    settings: {
        # a default project selector to use when none is specified
        # defaultSelector: <any selector>,
        # named project selectors for reuse
        selectors: {},
         # workspace log level if not overriden with the CLI 
        logLevel: 'Warn',
        # parallelism level to use when executing tasks (for e.g when using the `run` or `spawn` commands) if not overidden with the CLI.
        parallelism: 'None',
        # parallelism level to use when resolving executors
        resolutionParallelism: 'None'
    }
}
