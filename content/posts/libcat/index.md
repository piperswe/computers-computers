+++
title = "How I went overboard cataloging my library"
date = "2021-09-30"
author = "Piper McCorkle"
cover = "images/cover.jpg"
tags = ["home library", "books", "how-i-went-overboard"]
keywords = []
description = "I own quite a few books. I decided to go completely overboard on cataloging them, and chronicle my process here."
showFullContent = false
readingTime = true
hideComments = false
draft = false
+++

## Introduction

I own quite a few books. I haven't finished cataloging yet, so I don't know
exactly how many, but it's measured in the hundreds. A typical home library
probably consists of a few bookshelves, with books ordered in whatever way the
owner considers aesthetically pleasing.

{{< img
  src="images/stock_bookshelf.jpg"
  alt="A spartan bookshelf in a home with books and trinkets"
  caption="Photo by Vladimir Mokry / Unsplash"
>}}

That's a fantastic way of putting together a small home library, but it doesn't
scale well past 1 or 2 bookshelves, and I happen to like going overboard on
things. I decided to dedicate a whole room of my 1500sqft house to be my
library/reading room and catalog my books as if they were in an academic
library. That meant adopting a call number system, a catalog management system,
and possibly a circulation system. Thankfully, there's a great free website
called LibraryThing that makes this sort of thing super accessible, but I'll
talk more about that later.

## Call Numbers

{{< img
  src="images/call_numbers.jpg"
  alt="5 books sitting on a bookshelf, each with stickers on their spines showing a Library of Congress Classification number"
  caption="Some call numbers on the sides of some of my books"
>}}

If you've ever gone to a library, you're probably familiar with the Dewey
Decimal Classification. In a gist, it classifies books into a hierarchy and
gives parts of that hierarchy numerical identifiers. For instance, the 5xx
section is "Pure Science," the 51x section is Mathematics, 512 is Algebra,
512.7 is Number Theory, and it keeps going. Call number systems like the DDC
are great ways to sort books (and other media) in such a way that a patron can
easily find many books about the same topic in one place.

Unfortunately, the DDC has some issues.

First of all, it's proprietary. If I want the full documentation about the DDC
classifications, I need to either subscribe to OCLC's WebDewey, buy the 4
hardcover volumes of the latest revision of the DDC for nearly $400, or use a
community re-creation such as LibraryThing's
[Melvil Decimal System](https://www.librarything.com/mds). None of those seem
like great choices for a personal library - just think about how many other
books $400 can buy!

Second, it's a product of its time and hasn't changed as society has. For
instance, it still has classifications under topics such as "social problems"
and "sexual practices viewed as medical disorders" for media about GSRM/LGBT
topics! It also has an obvious Christian bias - most of the 2xx "Religion"
section is dedicated to Christianity, with other religions being shoved under
29x "Other Religions".

If the Dewey Decimal Classification were the only largely used call number
system out there, then I guess it would be fine. But we have alternatives! I
chose to go with the call number system used by and developed by the largest
library in the world, the Library of Congress Classification.

The LCC is available for free as
[FreeLCC](https://www.loc.gov/aba/publications/FreeLCC/freelcc.html), so I can
use the official LCC classifications without needing to pay a cent. If I
wanted, I could subscribe to [Classification Web](https://classweb.org/) for a
powerful interface to work with the LCC, but that's completely optional.

As for social issues, the Library of Congress seems more open to discontinuing
classifications that are made irrelevant and replacing them with alternatives.
For instance, they moved GSRM topics to be under "Sexual minorities" rather
than "Sexual deviations" in the 70s, and have since removed the "Sexual
deviations" category entirely. That's not to say the LCC is flawless in this
regard, but it seems to have done more work over time to reduce bias and
recategorize media than the DCC, which seems to not like recategorization.

## Catalog Management

{{< img
  src="images/librarything.png"
  alt="A screenshot of librarything.com/home with me logged in, showing my catalog and recommendations for me"
  caption="My LibraryThing profile, which manages my book catalog"
>}}

Just putting call numbers on each book would be a great step forward, but
that's only one part of cataloging. I need to actually keep track of what
books I have in my library.

There are quite a few catalog management systems out there.

- Libib is the first result from a Google search and seems to be a for-profit
  offering. It supports circulation, LCC, and barcode generation, but some of
  these features are locked behind the $9/mo "Pro" tier.
- Librarika looks like it could be a good system, and the 2,000 book limit on
  the free tier is probably plenty. I found this after I had already settled
  on LibraryThing.
- Koha is a self-hosted, open-source system. This would normally be right up
  my alley, but I also found this after I had already settled on LibraryThing.
  It seems to support all the features I'm looking for too... maybe someday
  I'll look into switching.

I ended up settling on LibraryThing after seeing recommendations online. Last
year, they [made the LibraryThing product free for everyone](https://blog.librarything.com/2020/03/librarything-goes-free/)
since their enterprise products (book recommendation engine and reviews based
on LibraryThing collections, TinyCat circulation system) were bringing in the
bulk of their revenue. LibraryThing supports every feature I'm looking for, and
also brings in a bit of a social component in the form of book reviews,
crowdsourced book information, forums, and probably more.

My process for cataloging a book with LibraryThing starts with the mobile app.
I scan the ISBN barcode with my phone, and LibraryThing will try to
automatically find the book in whichever databases I select for it to draw
from. I personally have it search the Library of Congress, Overcat (their
aggregation of lots of public & academic libraries), and Amazon.com, in that
order. If it doesn't find the book, then I set it aside for me to catalog on my
computer later.

With the "unknowns," I start by searching for the book on the
[Library of Congress's OPAC](https://catalog.loc.gov/) (or "Open Public Access
Catalog"). If they have it, then I copy the LCCN from the LC catalog, paste it
into LibraryThing, and it usually pulls the data from the LC catalog just fine.
If they don't have it, then I use LibraryThing's search tools to find the book
on some other database, like Overcat or Amazon. It's a bit of manual work, but
it's worth it to make sure my catalog is accurate. For any books that need some
TLC, I add them to my
["Needs Love"](https://www.librarything.com/catalog/pmc_/needslove) collection
so I know I need to get around to mending them later.

Once I'm done with a batch of books, I go through and add every children's book
to my [Juvenile collection](https://www.librarything.com/catalog/pmc_/juvenile).
I put those books on a separate set of shelves, so it helps to be able to see
where they are. This also helps with the TinyCat patron interface, which I'll
talk about later. Then, I [export my library as an Excel file](https://www.librarything.com/export.php?export_type=xls),
open that file, remove every column except for the barcode number and the LC
classification, remove rows for books that already have barcode stickers, then
plug the Excel file into Avery Design & Print Online's mail merge feature. I
set up AD&PO to print a barcode on about 3/4 of an address label, with the
right quarter having the call number. I then print that design on Avery 18260
address labels and put them on the books. On thick enough books, I put the
label on such that the left 3/4 is on the back and the right quarter is on the
spine (so you can see the call number on the spine), while on thinner books I
typically put the right quarter on the front and wrap the blank space around
the spine. On medium thickness books, I tend to keep the call number on the
spine and just have it wrap around a bit on the edges.

{{< img
  src="images/filled_shelf.png"
  alt="14 books sitting on a bookshelf, each labelled with a call number label as described above."
  caption="A small part of my book collection, post-cataloging"
>}}

When I order a book or decide I want to order one, I preliminarily add it to my
["Coming Soon" collection](https://www.librarything.com/catalog/pmc_/comingsoon)
and set its circulation status as "Ordered, Awaiting Delivery," "To be Ordered
Soon," or something to that effect. By setting that as its circulation status,
it shows that text in TinyCat and doesn't allow patrons to check it out or put
it on hold. But again, more on that in the next section.

I end up with a [public profile](https://www.librarything.com/profile/pmc_) on
the LibraryThing website which lets anyone see my library. This might not be
for everyone, but I quite like it. I do the same sort of thing with my music
through my [MusicBrainz collection](https://musicbrainz.org/collection/7a8bdef2-e937-42ac-b883-43b739b32798).
Maybe I'll do a post on my music catalog at some point in the future!

## Circulation System

{{< img
  src="images/tinycat.png"
  alt="A screenshot of librarycat.org/lib/pmc_"
  caption="The public homepage of my circulation system"
>}}

This is possibly the least important of these three sections. A circulation
system allows me to lend out books to patrons and keep track of who has what.
In a library the size of mine with the number of patrons I have (my housemates,
some family, and some friends) this is absolutely unnecessary but it makes me
feel like I'm running a bigger library than I actually am. It's sorta like a
big kid version of selling gum on the playground and pretending to operate it
like your little kid brain assumes a convenience store would operate.

At least I didn't have to really sink any time into this, since LibraryThing
seamlessly integrates into one of their other products, TinyCat. Even better,
TinyCat is completely free for personal use, no matter how many books you have!
(Though the software might have a limit of 20,000 books no matter your plan...
that's probably plenty.) TinyCat provides an OPAC along with a login system for
patrons to log in and self check-out or place holds. This is exactly the sort
of system that works great for my library. I can direct people to go visit
[librarycat.org/lib/pmc\_](https://librarycat.org/lib/pmc_) to see what books I
have, and my housemates and regular users have patron accounts so they can
check out books and I'm not wondering where a book wandered off to. It's really
as simple as connecting TinyCat to your LibraryThing account and scrolling
through the short list of settings to see if there's anything you want to
customize.

I think that's the gist of my cataloging efforts so far! I'm up to 165 books
cataloged, which is most of my juvenile collection, and it's been a relatively
smooth process! If any changes happen, I might come back and keep y'all up to
date... honestly depends on whether I remember.
