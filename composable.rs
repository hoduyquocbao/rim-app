use ahash::HashMapExt;
use ahash::HashMap;
// Định nghĩa trait `Component` là cấu trúc mà mọi thành phần trong ứng dụng sẽ triển khai
trait Component {
    fn render(&self) -> String;
}

// Một `Router` đơn giản sẽ quản lý việc chuyển đổi giữa các `Component`
struct Router {
    routes: HashMap<String, Box<dyn Component>>,
}

impl Router {
    // Khởi tạo `Router` với các `routes` rỗng
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    // Thêm một `route` mới
    fn add_route(&mut self, path: String, component: Box<dyn Component>) {
        self.routes.insert(path, component);
    }

    // Render `Component` dựa trên `path`
    fn navigate(&self, path: &str) -> String {
        match self.routes.get(path) {
            Some(component) => component.render(),
            None => format!("Không tìm thấy đường dẫn: {}", path),
        }
    }
}

// Ví dụ về một `Component` cụ thể
struct HomePage;

impl Component for HomePage {
    fn render(&self) -> String {
        String::from("Đây là Trang Chủ")
    }
}

// Hàm `main` để chạy ứng dụng
fn main() {
    let mut router = Router::new();
    router.add_route(String::from("/"), Box::new(HomePage));

    // Giả sử người dùng muốn điều hướng đến Trang Chủ
    println!("{}", router.navigate("/"));
}
