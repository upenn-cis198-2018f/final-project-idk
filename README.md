## Rust Cal Watcher

Rust Cal Watcher is a google-calendar centric utility meant to make scheduling appointments easier. It acts as a keylogger, waiting for the trigger key pattern ("\\" by default) to be pressed, at which point, it logs all keys pressed before a second input of the trigger key pattern. If valid, the resulting input is then made into a google calendar event and added to the user's calendar.

## State of the project

Currently, Rust Cal Watcher is in its infancy. The key-logging is only supported on true linux devices, and notifications still fail to function in most cases due to platform issues. However, RCW is also poised to be developed quickly. Code is isolated between the 3 layers (logger, parser, and server), allowing for logical changes to be made at any level without worry of the system falling apart.

Currently, RCW utilizes only the most basic of command parsing for determining if the string input is a valid event format. However, given the simplistic API and the modularity of the code, it would be easy to substitute in a more complex NLP based system.

## Code Structure

The code is made up of 3 major portions:
* The key logger:
  * This system is responsible for monitoring system input directly
  * It acts as a key logger, meaning that it is capable of detecting input in any window as long as the program is running
* The parser
  * This system parses the string from the key logger into an event which can be used by google calendar
  * Extremely modular and therefore easy to update with increasingly complex parser models
* The server
  * This portion of the system is responsible for all notifications issued by the system, both to google calendar and otherwise
  * Interacts with google's API, including Oauth2, as well as with the system.
  * Has the skeleton to begin notifications, but currently they are largely buggy depending on the system on which they are used.
  
## Team

Eric Frank

John Powell

Peter Chou
