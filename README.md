# hermes-proto

Prototyping a better emergency room triaging scheduler.

```
This prototype is currently pre-alpha.
```

## Background

Even before the COVID-19 pandemic, hospital emergency departments (ED) have been under extraordinary strain, with a significant lack of healthcare providers and resources [[1]](#1). However, the current status quo for triage scheduling for ambulatory patients is a single-source triage assessment, usually done by a triage nurse. In Canada, patients in triage are categorized into five levels: Resuscitation, Emergent, Urgent, Less Urgent, and Non-Urgent, as defined by the Canadian Triage and Acuity Scale (CTAS) [[2]](#2). Various other countries implement a similar categorization scheme based on chief complaint and symptoms, including Simple Triage and Rapid Treatment (START) in the U.S. and the Australasian Triage Scale (ATS) in Australia [[3]](#3).

Despite this categorization, emergency wait times for incoming patients exceed two hours on average [[4]](#4)[[5]](#5). Indeed, anecdotal evidence for wait times, not just for the ED but for other clinics well exceed the two hour average [[6]](#6)[[7]](#7), although it should be noted that patients that have a pleasant experience in the healthcare system are less likely to post about it online, while patients that experience poor performance are more likely to do so. As it stands, a pleasant experience in the healthcare system is not the norm, and the primary factor to a poor experience is wait time [[8]](#8).

## Exploratory Solution

Here we propose a new triage paradigm using a toy example simulator, which not only includes a severity score analogous to the triage scales as described above, but also takes into account patient wait time and estimated time-to-treat. This idea is inspired by CPU scheduling algorithms, where severity score is analogous to priority scheduling, wait time is analogous to first-come-first-serve (FCFS) scheduling and time-to-treat is analogous to shortest-job-first (SJF) scheduling.

We propose a hybrid scheduler that uses a `position_score` function incorporating severity, wait time, and time-to-treat as variables with arbitrary coefficients that can be tuned. In this toy example, we will be setting the `position_score` function as

$$\text{position score} = 0.5 \times \text{severity score} + 0.3 \times \text{wait time} + 0.2 \times \text{time to treat}$$

where

- `severity_score` is a arbitrary integer between [0, 100], with higher being more severe. In this prototype, `severity_score` will be randomly assigned.
- `wait_time` is a positive integer, representing minutes elapsed since arrival.
- `time_to_treat` is a positive integer, representing estimated minutes for patient turnaround, either to be admitted as an in-patient or discharge. In this prototype, `time_to_treat` will be randomly assigned.

Additionally, we propose that as wait time increases, that severity score likewise increases. For example:

```python
# Increase severity_score by one point every 15 minutes of wait time
severity_score = severity_score + (wait_time) // 15
```

This "upgrade" of severity score as a function of wait time is arbitrarily implemented. Various other implementations are up to the creativity of the implementer.

## Documentation

Documentation on the implementation details of `hermes-proto` can be found in [`/docs`](./docs/).

### Architecture

High-level architectural design can be found in [`./docs/design/architecure`](./docs/design/architecture/).

## References

<a id="1">[1]</a>
Alan Drummond, (2022). State of Emergency: Inside Canada's ER Crisis. Maclean's. https://macleans.ca/longforms/er-doctor-healthcare-crisis-canada/

<a id="2">[2]</a>
The Canadian Triage and Acuity Scale: Education Manual (2012). https://caep.ca/wp-content/uploads/2017/06/module_1_slides_v2.5_2012.pdf

<a id="3">[3]</a>
Charles Yancey, Maria C. O'Rourke, (2023). Emergency Department Triage. StatPearls. https://www.ncbi.nlm.nih.gov/books/NBK557583/

<a id="4">[4]</a>
Time Spent in Emergency Departments. Health Quality Ontario. https://www.hqontario.ca/System-Performance/Time-Spent-in-Emergency-Departments

<a id="5">[5]</a>
Emergency Department Wait Times. University Health Network. https://www.uhn.ca/PatientsFamilies/Visit_UHN/Emergency/Pages/ED_wait_times.aspx

<a id="6">[6]</a>
"Healthcare rant". Reddit. https://www.reddit.com/r/ontario/comments/1aw9mz7/healthcare_rant/

<a id="7">[7]</a>
"I love watching where I grew up crumble to greed". Reddit. https://www.reddit.com/r/ontario/comments/1c35eq8/i_love_watching_where_i_grew_up_crumble_to_greed/

<a id="8">[8]</a>
Mike Crawley, (2022). Patients tell the inside story of Ontario's emergency rooms. CBC. https://www.cbc.ca/news/canada/toronto/ontario-emergency-room-stories-1.6509555
