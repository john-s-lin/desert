from diagrams import Diagram
from diagrams.c4 import Container, Person, Relationship, SystemBoundary

with Diagram(
    name="DeSERT Hybrid Single-Queue",
    filename="../assets/desert-single-queue",
    outformat="svg",
    show=False,
):
    with SystemBoundary("PatientStream"):
        patient = Person(name="Patient", description="Asynchronous arrival")

    queue = Container("Hybrid Priority Queue")

    with SystemBoundary("DoctorQueue"):
        doctor = Person(name="Doctor", description="FCFS queue")

    patient >> Relationship("Enters triaging queue for processing") >> queue
    queue >> Relationship("Highest priority patient is seen") >> doctor

with Diagram(
    name="DeSERT Multi-Queue",
    filename="../assets/desert-multi-queue",
    outformat="svg",
    show=False,
):
    with SystemBoundary("PatientStream"):
        patient = Person(name="Patient", description="Asynchronous arrival")

    with SystemBoundary("MultiQueue"):
        priority_queue = Container(
            "Queue Level 1", description="Severity score-based priority queue"
        )
        fcfs_queue = Container(
            "Queue Level 2", description="FCFS queue based on time-of-arrival"
        )
        sjf_queue = Container(
            "Queue Level 3", description="SJF queue based on time-to-treat"
        )

    with SystemBoundary("DoctorQueue"):
        doctor = Person(name="Doctor", description="FCFS queue")

    patient >> Relationship() >> priority_queue
    patient >> Relationship() >> fcfs_queue
    patient >> Relationship() >> sjf_queue

    priority_queue >> Relationship() >> doctor
    (
        fcfs_queue
        >> Relationship("Consensus among queues for position closest to front of queue")
        >> doctor
    )
    sjf_queue >> Relationship() >> doctor
