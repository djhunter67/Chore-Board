# Purpose
Use my skills as a web developer to build a web app that ingests will keep track of the household chores for my family.  There are 11 children in the household at this time.  9 of the children are old enough to have a household task.  Some of the household tasks are not fit for the younger children.  There are no twins, fratenal or otherwise.

## Features
- [ ] List the household chores
- [ ] Assign the children to the household chores
- [ ] Track the completion of the household chores
- [ ] Automatically rotate the child assigned to the household chores
- [ ] Allow to easily add a new household chore
- [ ] Allow to easily add a new child
- [ ] Track the history of the household chores
- [ ] Locally host to the world ([web_app](household-chores.hunter-homelab.com))
  - [ ] Allow for multiple users to access the same household chores
  - [ ] Allow for multiple households to access the same household chores
  - [ ] Authentication and authorization
  - [ ] User management
  - [ ] Partial local-first design


## Architecture
To keep any users with minimal computer or web application experience from being overwhelmed the bones of the application will be simple and to the point. The application will be backend heavy.  The pattern of use will be mirrored for every action for which data is submitted.

## Design
There will be an aim to keep buttons, cards, and shapes uniform and aligned throughout the user experience. Text that will be a link will be uppercase and no boxes will have sharp corners.  Colors will be soft and text will be high contrast to its background. 

### User Experience
Attention to few on screen distractions will be paid.  There are currently no plans to gamify, thus there will be little to minimal effort to implement frills, poppers, etc... 

## Design Ideology
Optimizing for future expansion and a simplistics is the primary concern while developing due to the fact that the intended user group are non-technical family members.  Inherent in the design age group is the number of intended users is one but design will support multiple users.  Thus, simple expansion to different, more, or complex subjects will be attainable with minimal time and effort. This is a golden opportunity to roll a custom auth to know if that is something to be avoided in the future.

## Technologies
### Frontend
- HTML
- SCSS
- HTMX
- Javascript

The technologies used in the frontend are the __best tools for the job__.  To be clear, the __best tool for the job__ is compeletely subjective.  The __best tool for the job__ is the tool that is most familiar to the developer.  It is believed that familiarization with tools upon which most other tools are built is critical to a well-rounded developer.  Thus, the __best tools for the job__ are HTML, SCSS, and HTMX.  Experience with Bulma, Tailwind, and Bootstrap are strongly opinionated libraries and, admittedly, would work well in this scenario but there is more to learn abut CSS and every project is an opportunity to grow.

### Backend
- Rust
- Actix-Web
- SQLite
- Redis
- Linux (Arch, BTW, for development)

Learning about the key differences in practice versus reading about the differences will allow for a more well-rounded developer.  Rust is a language that is not only fun to write but is also a language that is type safe and fast. As a developer, continuous learning is a major part of the identity.

