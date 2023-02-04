import docker
DOCKER_CLIENT = docker.DockerClient(base_url='unix://var/run/docker.sock')
RUNNING = 'running'

# src: https://stackoverflow.com/questions/35586900/how-to-check-if-a-docker-instance-is-running
def is_running(container_name):
    """
    verify the status of a sniffer container by it's name
    :param container_name: the name of the container
    :return: Boolean if the status is ok
    """
    container = DOCKER_CLIENT.containers.get(container_name)

    container_state = container.attrs['State']

    container_is_running = container_state['Status'] == RUNNING

    return container_is_running