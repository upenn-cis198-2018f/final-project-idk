## Rust Cal Watcher

Rust Cal Watcher is a google-calendar centric utility meant to make scheduling appointments easier. It acts as a keylogger, waiting for the trigger key pattern ("//" by default) to be pressed, at which point, it logs all keys pressed before a second input of the trigger key pattern. If valid, the resulting input is then made into a google calendar event and added to the user's calendar.

## State of the project

RCW is poised to be developed quickly. Code is isolated between the 3 layers (logger, parser, and server), allowing for logical changes to be made at any level without worry of the system falling apart. Additionally, a fourth system, the Notifyer, is available to customize the means through which notifications are delivered to the end user.

Currently, RCW utilizes regex-based and keyword driven text parsing. However, given the simplistic API and the modularity of the code, it would be easy to substitute in an even more complex system, such as an NLP powered one.

## Code Structure

The code is made up of 4 major portions:
* The key logger:
  * This system is responsible for monitoring system input directly
  * It acts as a key logger, meaning that it is capable of detecting input in any window as long as the program is running
  * Automatically detects default user keyboard
* The parser
  * This system parses the string from the key logger into an event which can be used by google calendar
  * Uses basic heuristics to determine a start time, end time, location, and description of an event based on user input
  * Extremely modular and therefore easy to update with increasingly complex parser models
  * All features are unit tested
* The server
  * This portion of the system is responsible for all notifications issued by the system, both to google calendar and otherwise
  * Interacts with google's API, including Oauth2, as well as with the system.
  * Has the skeleton to begin notifications, but currently they are largely buggy depending on the system on which they are used.
* The Notifyer:
  * This portion is responsible for delivering notifications to the end user
  * utilizes traits to regularizes the API
  
## Team

Eric Frank: Server

John Powell: Parser

Peter Chou: Logger
