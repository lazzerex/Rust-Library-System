use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    isbn: String,
    available: bool,
    due_date: Option<u64>,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Book: {} by {} (ISBN: {}) - {}",
            self.title,
            self.author,
            self.isbn,
            if self.available {
                "Available"
            } else {
                "Checked Out"
            }
        )
    }
}

#[derive(Debug)]
struct Member {
    id: u32,
    name: String,
    borrowed_books: Vec<u32>,
}

impl Member {
    fn new(id: u32, name: String) -> Self {
        Member {
            id,
            name,
            borrowed_books: Vec::new(),
        }
    }
}

struct Library {
    books: HashMap<u32, Book>,
    members: HashMap<u32, Member>,
    next_book_id: u32,
    next_member_id: u32,
}

impl Library {
    fn new() -> Self {
        Library {
            books: HashMap::new(),
            members: HashMap::new(),
            next_book_id: 1,
            next_member_id: 1,
        }
    }

    fn add_book(&mut self, title: String, author: String, isbn: String) -> u32 {
        let book = Book {
            id: self.next_book_id,
            title,
            author,
            isbn,
            available: true,
            due_date: None,
        };
        self.books.insert(self.next_book_id, book);
        self.next_book_id += 1;
        self.next_book_id - 1
    }

    fn add_member(&mut self, name: String) -> u32 {
        let member = Member::new(self.next_member_id, name);
        self.members.insert(self.next_member_id, member);
        self.next_member_id += 1;
        self.next_member_id - 1
    }

    fn check_out_book(&mut self, book_id: u32, member_id: u32) -> Result<(), String> {
        // Check if book and member exist
        if !self.books.contains_key(&book_id) {
            return Err("Book not found".to_string());
        }
        if !self.members.contains_key(&member_id) {
            return Err("Member not found".to_string());
        }

        // Check if book is available
        let book = self.books.get_mut(&book_id).unwrap();
        if !book.available {
            return Err("Book is not available".to_string());
        }

        // Calculate due date (2 weeks from now)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let two_weeks = 60 * 60 * 24 * 14;

        // Update book status
        book.available = false;
        book.due_date = Some(now + two_weeks);

        // Add book to member's borrowed books
        let member = self.members.get_mut(&member_id).unwrap();
        member.borrowed_books.push(book_id);

        Ok(())
    }

    fn return_book(&mut self, book_id: u32, member_id: u32) -> Result<(), String> {
        // Validate book and member
        if !self.books.contains_key(&book_id) {
            return Err("Book not found".to_string());
        }
        if !self.members.contains_key(&member_id) {
            return Err("Member not found".to_string());
        }

        // Check if member has borrowed this book
        let member = self.members.get_mut(&member_id).unwrap();
        if !member.borrowed_books.contains(&book_id) {
            return Err("This member has not borrowed this book".to_string());
        }

        // Update book status
        let book = self.books.get_mut(&book_id).unwrap();
        book.available = true;
        book.due_date = None;

        // Remove book from member's borrowed books
        member.borrowed_books.retain(|&x| x != book_id);

        Ok(())
    }

    fn get_member_books(&self, member_id: u32) -> Result<Vec<&Book>, String> {
        match self.members.get(&member_id) {
            Some(member) => {
                let books: Vec<&Book> = member
                    .borrowed_books
                    .iter()
                    .filter_map(|book_id| self.books.get(book_id))
                    .collect();
                Ok(books)
            }
            None => Err("Member not found".to_string()),
        }
    }

    fn search_books(&self, query: &str) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| {
                book.title.to_lowercase().contains(&query.to_lowercase())
                    || book.author.to_lowercase().contains(&query.to_lowercase())
                    || book.isbn.contains(query)
            })
            .collect()
    }
}

fn main() {
    let mut library = Library::new();

    // Add some books
    let book1_id = library.add_book(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik".to_string(),
        "978-1593278281".to_string(),
    );
    let book2_id = library.add_book(
        "Zero To Production In Rust".to_string(),
        "Luca Palmieri".to_string(),
        "978-3001234567".to_string(),
    );

    // Add a member
    let member_id = library.add_member("John Doe".to_string());

    // Demonstrate book checkout
    match library.check_out_book(book1_id, member_id) {
        Ok(_) => println!("Book checked out successfully"),
        Err(e) => println!("Error checking out book: {}", e),
    }

    // Search for books
    println!("\nSearching for 'Rust' books:");
    for book in library.search_books("Rust") {
        println!("{}", book);
    }

    // Get member's borrowed books
    match library.get_member_books(member_id) {
        Ok(books) => {
            println!("\nJohn Doe's borrowed books:");
            for book in books {
                println!("{}", book);
            }
        }
        Err(e) => println!("Error getting member's books: {}", e),
    }

    // Return the book
    match library.return_book(book1_id, member_id) {
        Ok(_) => println!("\nBook returned successfully"),
        Err(e) => println!("Error returning book: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_library() -> (Library, u32, u32) {
        let mut library = Library::new();
        let book_id = library.add_book(
            "Test Book".to_string(),
            "Test Author".to_string(),
            "123-4567890".to_string(),
        );
        let member_id = library.add_member("Test Member".to_string());
        (library, book_id, member_id)
    }

    #[test]
    fn test_add_book() {
        let mut library = Library::new();
        let book_id = library.add_book(
            "Test Book".to_string(),
            "Test Author".to_string(),
            "123-4567890".to_string(),
        );
        
        assert_eq!(book_id, 1);
        let book = library.books.get(&book_id).unwrap();
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.author, "Test Author");
        assert_eq!(book.isbn, "123-4567890");
        assert!(book.available);
        assert!(book.due_date.is_none());
    }

    #[test]
    fn test_add_member() {
        let mut library = Library::new();
        let member_id = library.add_member("Test Member".to_string());
        
        assert_eq!(member_id, 1);
        let member = library.members.get(&member_id).unwrap();
        assert_eq!(member.name, "Test Member");
        assert!(member.borrowed_books.is_empty());
    }

    #[test]
    fn test_check_out_book_success() {
        let (mut library, book_id, member_id) = setup_library();
        
        let result = library.check_out_book(book_id, member_id);
        assert!(result.is_ok());
        
        let book = library.books.get(&book_id).unwrap();
        assert!(!book.available);
        assert!(book.due_date.is_some());
        
        let member = library.members.get(&member_id).unwrap();
        assert!(member.borrowed_books.contains(&book_id));
    }

    #[test]
    fn test_check_out_book_not_found() {
        let (mut library, _, member_id) = setup_library();
        
        let result = library.check_out_book(999, member_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Book not found");
    }

    #[test]
    fn test_check_out_book_not_available() {
        let (mut library, book_id, member_id) = setup_library();
        
        // Check out the book first
        library.check_out_book(book_id, member_id).unwrap();
        
        // Try to check out the same book again
        let result = library.check_out_book(book_id, member_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Book is not available");
    }

    #[test]
    fn test_return_book_success() {
        let (mut library, book_id, member_id) = setup_library();
        
        // Check out the book first
        library.check_out_book(book_id, member_id).unwrap();
        
        // Return the book
        let result = library.return_book(book_id, member_id);
        assert!(result.is_ok());
        
        let book = library.books.get(&book_id).unwrap();
        assert!(book.available);
        assert!(book.due_date.is_none());
        
        let member = library.members.get(&member_id).unwrap();
        assert!(!member.borrowed_books.contains(&book_id));
    }

    #[test]
    fn test_return_book_not_borrowed() {
        let (mut library, book_id, member_id) = setup_library();
        
        // Try to return a book that wasn't checked out
        let result = library.return_book(book_id, member_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "This member has not borrowed this book");
    }

    #[test]
    fn test_search_books() {
        let mut library = Library::new();
        library.add_book(
            "Rust Programming".to_string(),
            "Author One".to_string(),
            "111-1111111".to_string(),
        );
        library.add_book(
            "Python Programming".to_string(),
            "Author Two".to_string(),
            "222-2222222".to_string(),
        );
        
        let rust_books = library.search_books("Rust");
        assert_eq!(rust_books.len(), 1);
        assert_eq!(rust_books[0].title, "Rust Programming");
        
        let author_books = library.search_books("Author");
        assert_eq!(author_books.len(), 2);
    }

    #[test]
    fn test_get_member_books() {
        let (mut library, book_id, member_id) = setup_library();
        
        // Initially no books
        let result = library.get_member_books(member_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
        
        // Check out a book
        library.check_out_book(book_id, member_id).unwrap();
        
        // Should now have one book
        let result = library.get_member_books(member_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_get_member_books_member_not_found() {
        let (library, _, _) = setup_library();
        
        let result = library.get_member_books(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Member not found");
    }
}