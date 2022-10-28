+++
title = 'Syncy Workflows - A Concept'
date = "2022-10-16"
author = "Piper McCorkle"
cover = ""
tags = ["concept"]
keywords = []
description = "A concept for building offline-first networked applications"
showFullContent = false
readingTime = true
hideComments = false
draft = true
+++

There's a forgotten art of making applications that progressively enhance based on network connection status,
especially on desktop. This recently became a pain in my side when I wanted to write and turn in a homework assignment
while on a plane ride, but I realized I couldn't just queue up an upload to happen when I came back online. This
functionality may be less necessary in today's world of constant connectivity, but I think there is still value in
considering fluctuating connectivity in software design.

When considering what sort of architecture could make these applications work, I thought back to e-mail. Originally,
consumer e-mail was designed with dial-up internet in mind: you logged in to your e-mail server and your client
automatically downloaded any new messages and sent any messages that were queued in the outbox, then you could
disconnect from the Internet and read your messages offline without consuming precious dial-up minutes.

## "Syncy" Workflows?

Oftentimes, networked applications today behave as a dumb terminal for a remote server - requesting information only
when it's needed and expecting any interactivity to be immediately acted on by the remote server. I propose an
alternative to these "chatty" workflows with "syncy" workflows. Unlike these chatty workflows that are constantly
chatting with a remote server and have no information to display without that server, a syncy workflow involves
distinct download and upload operations.

In a syncy workflow, an application may behave just the same to the user when online, but in the background it's
downloading all relevant state from the server so the user has access to it when connectivity is limited. The
application doesn't need a round-trip to the server for every navigation action, since it should already have that data
downloaded. This downloading may be a continuous stream of changes while online, scheduled bursts of changes, or
manually triggered by the user.

When the user makes a change, such as by uploading a piece of homework, that task gets placed in a queue to be uploaded
with the next synchronization. If the application is developed with the distinct actions of uploading and downloading
changes in this way in mind, it should be able to gracefully handle low-connectivity situations.

## Possible Implementation

I've brainstormed a possible implementation that a whole system built around syncy workflows could use. This design
is overkill for a single application, but could provide a great user experience for a system built around syncy
applications. 

This possible implementation isn't a specification for a real implementation, it's just a brainstorm of what could be.

### Architecture

> `syncyd` daemon running "tasks," small scripts that do one atomic task over the network and produce a result

This fictional system would have a single global (or maybe user-specific) daemon (let's call it `syncyd` for brevity
later) which handles all network traffic, and applications would have no network access on their own. Applications
would talk to this daemon to coordinate their synchronization.

The root of the actual synchronization process is the "task" primitive. A task consists of a reference to a "task
runner" and input data for the task runner. When a task is run, it calls the task runner, passing it the input data.
The output of the task runner is saved by `syncyd` and returned to the application when it asks.

A task runner is a script which performs one atomic network operation. It can consist of multiple round-trips, if
that's what it takes to complete one logical task.

Tasks can be scheduled, either repeated or one-off, by creating a "schedule."

`syncyd` keeps track of network connectivity, so it queues tasks while offline and runs tasks when back online. If a
task fails, it gets retried up to the maximum allowed retries before getting a "fail" result. `syncyd` can also be
given a reachability probe endpoint & method (HTTP/ping) so a task is only run if the endpoint that particular task is
concerned with is reachable.

### API & Technical Specifics

> Connect to `syncyd`, and with that socket receive task result updates, create new tasks, and query task status

`syncyd` is exposed over D-Bus, so any application in the session can talk to it with ease. Applications are packaged
with Flatpak with no network access but are allowed to talk to `syncyd` over D-Bus.

#### Runner API

> JavaScript with some Web Platform goodies. Export `task` function that runs the task.

> TCP/UDP with the same API that Cloudflare is trying to build with Socket Workers

```ts
// An example HTTP task runner, written in TypeScript

// These interfaces are just defined for clarity, type info isn't consumed by syncyd

interface Input {
  url: string,
  fetchOptions: unknown
}

interface Result {
  code: number,
  body: string
}

// The exported runTask function takes the task's input and performs the task
export async function runTask({ url, fetchOptions }: Input): Promise<Result> {
  // Using standardized Web Platform APIs like fetch
  const response = await fetch(url, fetchOptions);
  // Whatever gets returned is serialized to JSON for the application to consume
  return {
    code: response.code,
    body: await response.text(),
  };
}
```

```yaml
# An example task using the above runner

# Runners are scoped to domains, similar to freedesktop.org IDs
runner: io.syncyd.examplehttprunner
input:
  url: https://www.google.com
  fetchOptions:
    method: GET
```

```yaml
# An example schedule based on the above task

cron: 
```

#### Included Runners

> - Basic HTTP request

### User Interface

> GUI which shows pending tasks and allows user to manually activate a task

## Prior Art

> Syncy workflows aren't a new concept, just a forgotten one

### E-mail

> IMAP, SMTP, POP3, etc. allow for syncy workflows.