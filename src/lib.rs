use cursive::traits::{Boxable, Scrollable};
use cursive::views::{Dialog, EditView, SelectView};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stores the State of the program and the user's data. Gets serialized after every change.
#[derive(Serialize, Deserialize)]
pub struct State {
    pub reviews: HashMap<usize, u8>,
}

pub struct ChapterInfo {
    pub human_readable: &'static str,
    extra_info: &'static str,
}

impl ChapterInfo {
    fn chapter_resources_string(&self) -> String {
        if self.extra_info.is_empty() {
           String::from("Make sure to look at the Rust by Example online book for some examples!")
        } else {
            format!("Check out these helpful resources:\n\n{}", self.extra_info)
        }
    }
}

/// Stores the name of all the rust book chapters. The indexes of each chapter string are refrenced throughout the program.
pub const CHAPTERS: [ChapterInfo; 97] = [
    ChapterInfo {
        human_readable: "1. Getting Started",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "1.1. Installation",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "1.2. Hello, World!",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "1.3. Hello, Cargo!",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "2. Programming a Guessing Game",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "3. Common Programming Concepts",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "3.1. Variables and Mutability",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "3.2. Data Types",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "3.3. Functions",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "3.4. Comments",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "3.5. Control Flow",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "4. Understanding Ownership",
        extra_info: "https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b\nhttps://dev.to/domcorvasce/learning-rust-1-ownership-g0",
    },
    ChapterInfo {
        human_readable: "4.1. What is Ownership?",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "4.2. References and Borrowing",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "4.3. The Slice Type",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "5. Using Structs to Structure Related Data",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "5.1. Defining and Instantiating Structs",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "5.2. An Example Program Using Structs",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "5.3. Method Syntax",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "6. Enums and Pattern Matching",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "6.1. Defining an Enum",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "6.2. The match Control Flow Operator",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "6.3. Concise Control Flow with if let",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "7. Managing Growing Projects with Packages, Crates, and Modules",
        extra_info: "https://stevedonovan.github.io/rust-gentle-intro/4-modules.html",
    },
    ChapterInfo {
        human_readable: "7.1. Packages and Crates",
        extra_info: "https://stevedonovan.github.io/rust-gentle-intro/4-modules.html",
    },
    ChapterInfo {
        human_readable: "7.2. Defining Modules to Control Scope and Privacy",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "7.3. Paths for Referring to an Item in the Module Tree",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "7.4. Bringing Paths Into Scope with the use Keyword",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "7.5. Separating Modules into Different Files",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "8. Common Collections",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "8.1. Storing Lists of Values with Vectors",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "8.2. Storing UTF-8 Encoded Text with Strings",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "8.3. Storing Keys with Associated Values in Hash Maps",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "9. Error Handling",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "9.1. Unrecoverable Errors with panic!",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "9.2. Recoverable Errors with Result",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "9.3. To panic! or Not To panic!",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "10. Generic Types, Traits, and Lifetimes",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "10.1. Generic Data Types",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "10.2. Traits: Defining Shared Behavior",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "10.3. Validating References with Lifetimes",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "11. Writing Automated Tests",
        extra_info: "https://medium.com/@KalROFL/plp-rust-agjks-395d1d870432",
    },
    ChapterInfo {
        human_readable: "11.1. How to Write Tests",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "11.2. Controlling How Tests Are Run",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "11.3. Test Organization",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12. An I/O Project: Building a Command Line Program",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12.1. Accepting Command Line Arguments",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12.2. Reading a File",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12.3. Refactoring to Improve Modularity and Error Handling",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12.4. Developing the Library’s Functionality with Test Driven Development",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12.5. Working with Environment Variables",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "12.6. Writing Error Messages to Standard Error Instead of Standard Output",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "13. Functional Language Features: Iterators and Closures",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "13.1. Closures: Anonymous Functions that Can Capture Their Environment",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "13.2. Processing a Series of Items with Iterators",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "13.3. Improving Our I/O Project",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "13.4. Comparing Performance: Loops vs. Iterators",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "14. More about Cargo and Crates.io",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "14.1. Customizing Builds with Release Profiles",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "14.2. Publishing a Crate to Crates.io",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "14.3. Cargo Workspaces",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "14.4. Installing Binaries from Crates.io with cargo install",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "14.5. Extending Cargo with Custom Commands",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "15. Smart Pointers",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "15.1. Using Box to Point to Data on the Heap",
        extra_info: "",
    },
    ChapterInfo {
        human_readable:
            "15.2. Treating Smart Pointers Like Regular References with the Deref Trait",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "15.3. Running Code on Cleanup with the Drop Trait",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "15.4. Rc, the Reference Counted Smart Pointer",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "15.5. RefCell and the Interior Mutability Pattern",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "15.6. Reference Cycles Can Leak Memory",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "16. Fearless Concurrency",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "16.1. Using Threads to Run Code Simultaneously",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "16.2. Using Message Passing to Transfer Data Between Threads",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "16.3. Shared-State Concurrency",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "16.4. Extensible Concurrency with the Sync and Send Traits",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "17. Object Oriented Programming Features of Rust",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "17.1. Characteristics of Object-Oriented Languages",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "17.2. Using Trait Objects That Allow for Values of Different Types",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "17.3. Implementing an Object-Oriented Design Pattern",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "18. Patterns and Matching",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "18.1. All the Places Patterns Can Be Used",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "18.2. Refutability: Whether a Pattern Might Fail to Match",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "18.3. Pattern Syntax",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "19. Advanced Features",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "19.1. Unsafe Rust",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "19.2. Advanced Traits",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "19.3. Advanced Types",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "19.4. Advanced Functions and Closures",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "19.5. Macros",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "20. Final Project: Building a Multithreaded Web Server",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "20.1. Building a Single-Threaded Web Server",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "20.2. Turning Our Single-Threaded Server into a Multithreaded Server",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "20.3. Graceful Shutdown and Cleanup",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "21.1. A - Keywords",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "21.2. B - Operators and Symbols",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "21.3. C - Derivable Traits",
        extra_info: "",
    },
    ChapterInfo {
        human_readable: "21.4. D - Useful Development Tools",
        extra_info: "",
    },
];

pub mod screens;

pub mod state;

/// Used to find the index of a user-inputted chapter name in the `CHAPTERS` array. Will not panic if it does not exist, returns `None` instead.
///
/// # Examples
/// ```
/// use rustacean_review::{CHAPTERS, find_chapter_index};
/// assert_eq!(Some(0), find_chapter_index("1. Getting Started", CHAPTERS));
/// ```
pub fn find_chapter_index(search_string: &str, chapters_list: [ChapterInfo; 97]) -> Option<usize> {
    for (index, chapter) in chapters_list.iter().enumerate() {
        if chapter.human_readable.starts_with(search_string) {
            return Some(index);
        }
    }

    None
}

/// Finds the lowest ranked chapter and returns the an Option with the index of the chapter in the `CHAPTERS` array.
///
/// # Examples
/// ```
/// use rustacean_review::{State, smallest_value_in_hashmap};
/// use std::collections::HashMap;
///
/// let mut hashmap_example = HashMap::new();
/// hashmap_example.insert(1, 10);
/// hashmap_example.insert(90, 1);
///
/// let lowest_ranked_chapter = smallest_value_in_hashmap(&hashmap_example).unwrap();
/// assert_eq!(lowest_ranked_chapter, 90);
/// ```

/// Returns the key corresponding to the SMALLEST value in a Hashmap.
pub fn smallest_value_in_hashmap(hashmap: &HashMap<usize, u8>) -> Option<usize> {
    let mut entry = (&0usize, &255);

    for (key, value) in hashmap {
        if value < entry.1 {
            entry.0 = key;
            entry.1 = value;
        }
    }

    // If tuple is still default state
    if entry == (&0usize, &255) {
        None
    } else {
        // If tuple changed:
        Some(*entry.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_chapter_index, smallest_value_in_hashmap, State, CHAPTERS};

    #[test]
    fn chapter_search() {
        assert_eq!(Some(94), find_chapter_index("21.2", CHAPTERS));
    }

    #[test]
    fn unknown_chapter_search() {
        assert_eq!(None, find_chapter_index("9999.9", CHAPTERS));
    }

    #[test]
    fn smallest_in_hashmap() {
        use std::collections::HashMap;

        let mut hashmap_example = HashMap::new();
        hashmap_example.insert(21, 9);
        hashmap_example.insert(44, 3);
        hashmap_example.insert(12, 7);
        hashmap_example.insert(99, 6);

        assert_eq!(smallest_value_in_hashmap(&hashmap_example).unwrap(), 44);

        hashmap_example.insert(90, 1);

        assert_eq!(smallest_value_in_hashmap(&hashmap_example).unwrap(), 90);
    }

    #[test]
    fn empty_smallest_in_hashmap() {
        use std::collections::HashMap;

        let hashmap_example = HashMap::new();
        assert_eq!(smallest_value_in_hashmap(&hashmap_example), None);
    }
}
