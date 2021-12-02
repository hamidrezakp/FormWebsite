## Case Info (1)
- id: u32
- status: Active | InActive
- registration date: datetime
- registered by: (string, telegram.uid) "admin name and userID"
- location: string

## Personal Information of leader (1)
- name: string
- father name: string
- birthday: datetime
- national number: u32
- phone number: string (maybe later phoneNumber)

## Career and educational information of leader (1)
- job: string
- income: u32
- job experiences: string list
- skills: string list
- educational field: string list
- educational location: string list

## Family information (N)
- name: string
- birthday: datetime
- national number: u32
- educational status: string
- skills: string list

## Description (1)
- text: string

## Requirements (N)
- text: string

## Follow up and Actions (N)
- description: string
- reminderDate: datetime option

## Debts (N)
- title: string
- duration: timespan
