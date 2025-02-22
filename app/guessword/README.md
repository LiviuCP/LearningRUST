1. INTRODUCTION

This application requires the user to guess a word. First the size should be guessed. Once this is done the word content should be discovered by user.

2. SUPPORTED OPERATING SYSTEMS

The application has been tested so far on Linux (OpenSUSE 15.4) and Mac (Monterey).

To run it just install the RUST environment (including Cargo: more details on the RUST webpage) and execute 'cargo run' from the guessword or guessword/src directories. Alternatively execute 'cargo build' and run the executable from the guessword/target directory. To run with options using Cargo enter: cargo run -- [options] (currently there is only the --entry option, see section 5).

3. RUNNING THE APPLICATION

When the application is launched from terminal without any options, it enters the main menu in which the user is asked to guess a word. For each word, there are two steps to be fulfilled: guessing the word size and guessing the actual word content.

First, the number of characters should be provided. If the number is not the right one, the user is advised whether the provided count should be lower or higher. Once the user guessed the number of characters next step is launched, namely guessing the actual word.

When guessing the content commences, only the first and the last letter are being initially displayed, the chars to guess being shown as underscore placeholders. The user is asked to provide a character. If the character is not contained within word or has already been guessed, a retry is requested. If the guess is correct the placeholders of the guessed char occurrences are being filled in. Another character is then requested. Once all chars have been revealed, the step ends and the cycle resumes until all available words got consumed by app or the user aborts the application.

An example could be guessing the word "pedestrian". Here is how the word is displayed after guessing each character:

Initially:        p________n

User guesses 'a': p_______an
User guesses 'e': pe_e____an
User guesses 'd': pede____an
User guesses 's': pedes___an
User guesses 'r': pedes_r_an
User guesses 'i': pedes_rian
User guesses 't': pedestrian

It's over, the word has been guessed!

Notes:
- currently there is an unlimited number of times a character can be entered until the whole word has been guessed
- entering the characters is case insensitive
- only alphabet characters are being allowed
- each word has a minimum size of 10 characters
- once a character has been guessed, all its occurrences are being filled in. When entering the same character again, the user is advised that the it has already been provided.
- the first and last character are provided for making life easier to the player. However any of them could still be contained behind placeholders.
- the words are being provided randomly to the user. However it should be noted that no word is being provided twice (provided no new words have been entered in the meantime - see section 5). Once all words have been guessed, the user is notified regarding this and the application ends.
- when the application is launched the main menu is not being displayed if there are no words available for guessing. See sections 4 and 5 for more details.

4. APPLICATION DATA FILE

The application data file is called input.txt and is contained within Documents folder from user home directory. When the application starts it checks for this file. If the file exists, the application attempts to load the file content. If no file is found, an empty data file is created within mentioned folder and the used should enter at least one new word in order to get to the main (guessing) menu (more details in section 5).

The file contains one word per row. Upon loading the content, the application checks each entry and adds it to the running data if it fulfills the requirements. These are:
- the word should contain minimum 10 characters
- the characters should be alphabetic (case doesn't matter)

If no words fulfill the above mentioned criteria, then the running data will be empty and the user will be redirected to the data entry menu (more about this in section 5). On the other hand if at least one word exists the main menu is entered the user is prompted to guess a randomly chosen word (more details in section 3).

5. DATA ENTRY

Currently the user has two ways to add data to the input.txt file:
- either (create it and) add words manually line by line using a text editor (it is obviously possible to also modify and/or delete words by using this method)
- or use the application data entry menu

This section focuses on the second method. When entering the data entry menu, all the user needs to do is enter each new word followed by the ENTER key. When no more words need to be added simply press the ENTER key again. A prompt will be displayed asking to confirm whether the data should be saved or not:
- when pressing y (yes) + ENTER: the data is saved and the user is brought to the main (guessing) menu
- when pressing n (no) + ENTER: the new data is discarded and the user is directed to the main menu
- when pressing c (cancel) + ENTER: the user is asked to enter a new word (the already added ones are kept within buffer)

The data entry menu is launched in three scenarios:
- when the user starts the application with the --entry option
- when the user starts the application without options but no words could be successfully loaded from data file
- when the user enter character ':' followed ENTER while in main (guessing) menu

If the user decided to exit the menu without successfully entering any new words, another prompt is displayed stating that no new words have been entered and asking if the user really wants to exit the menu.

Notes:
- when the user decides to save, a consolidation process takes place which considers both the already existing words and the new ones. The duplicate entries are getting discarded. If only duplicate new entries (comparing to already existing words) exist withing new words pool, then nothing is saved and the user gets notified.
- when each new word gets entered, same validity checks are being performed as when loading new words from data file (except for duplicates which are being checked only when saving to data file - see above). If for example the entered word has less characters than the minimum required, then the user is notified that the entry is invalid. The word will not get added to the dataset to be included in the saving process.
- if no words got loaded from data file and the user exited the data entry menu without saving any new words, then an error is triggered and the application exits
- if the user entered the data entry menu from main menu and no new words got saved at exit, then the main menu is restored in the same state it was before switching to data entry. For example if the user is about to guess the word "pedestrian" and the word is displayed "p______i_n" before deciding new words should be added to dataset, then the same progress will be displayed upon return. On the other hand, if the user saves new words then the guessing process will get restarted and a random word (from the whole list - including the new ones) will be chosen for guessing. The user should start by guessing its size!
- adding words is case insensitive
- the prompt for saving is also case insensitive
- the prompt for exiting the data entry menu when no new words were (successfully) entered is case insensitive as well
