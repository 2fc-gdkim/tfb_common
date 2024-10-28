#[derive(Debug, Clone)]
pub struct ByteBuffer {
    pub bytes: Vec<u8>,
    pub index: usize,
}

impl ByteBuffer {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            index: 0,
        }
    }
    pub fn new_with_capacity(length: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(length),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn rewind(&mut self) {
        self.index = 0;
    }

    //u8추가
    pub fn append_u8(&mut self, value: u8) {
        self.bytes.push(value);
        self.index += 1;
    }

    //u16추가
    pub fn append_u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());    //네트워크 바이트오더
        self.index += 2;
    }

    //u32추가
    pub fn append_u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());    //네트워크 바이트오더
        self.index += 4;
    }

    //u64추가
    pub fn append_u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());    //네트워크 바이트오더
        self.index += 8;
    }

    //i8추가
    pub fn append_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
        self.index += 1;
    }

    //i16추가
    pub fn append_i16(&mut self, value: i16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());    //네트워크 바이트오더
        self.index += 2;
    }

    //i32추가
    pub fn append_i32(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());    //네트워크 바이트오더
        self.index += 4;
    }

    //i64추가
    pub fn append_i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());    //네트워크 바이트오더
        self.index += 8;
    }    

    //String 추가
    pub fn append_string(&mut self, value: &str, length: usize) {
        let text_bytes = value.as_bytes();
    
        if text_bytes.len() < length { // 문자열의 크기가 length보다 작을 경우            
            let mut fixed_size_array: Vec<u8> = Vec::with_capacity(length);
            fixed_size_array.resize(length, 0);
            fixed_size_array[..text_bytes.len()].copy_from_slice(text_bytes);
            self.bytes.extend_from_slice(&fixed_size_array);
        } 
        else { // 문자열의 크기가 length보다 클 경우
            self.bytes.extend_from_slice(&text_bytes[..length]);
        }
        self.index += length as usize;
    }

    //vector 추가
    // pub fn append_vector(&mut self, value: &Vec<u8>) {
    //     self.bytes.extend_from_slice(value);
    //     self.index += value.len();
    // }

    //vector 추가
    pub fn append_bytes(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
        self.index += value.len();
    }

    //u8 데이터 가져오기
    pub fn get_u8(&mut self) -> u8 {
        if self.index + 1 <= self.bytes.len() {            
            let value = self.bytes[self.index];
            self.index += 1;
            value
        }
        else {
            0
        }
    }

    //u16 데이터 가져오기
    pub fn get_u16(&mut self) -> u16 {
        if self.index + 2 <= self.bytes.len() {
            let value = u16::from_be_bytes([self.bytes[self.index]
                , self.bytes[self.index+1]]); //네트워크 바이트오더를 숫자로 
            self.index += 2;
            value
        }
        else {
            0
        }
    }

    //u32 데이터 가져오기
    pub fn get_u32(&mut self) -> u32 {
        if self.index + 4 <= self.bytes.len() {
            let value = u32::from_be_bytes([self.bytes[self.index]
                , self.bytes[self.index+1]
                , self.bytes[self.index+2]
                , self.bytes[self.index+3]]); //네트워크 바이트오더를 숫자로 
            self.index += 4;
            value
        }
        else {
            0
        }
    }

    //u64 데이터 가져오기
    pub fn get_u64(&mut self) -> u64 {
        if self.index + 8 <= self.bytes.len() {
            let value = u64::from_be_bytes([self.bytes[self.index]
                , self.bytes[self.index+1]
                , self.bytes[self.index+2]
                , self.bytes[self.index+3]
                , self.bytes[self.index+4]
                , self.bytes[self.index+5]
                , self.bytes[self.index+6]
                , self.bytes[self.index+7]]); //네트워크 바이트오더를 숫자로 
            self.index += 8;
            value
        }
        else {
            0
        }
    }

    //i8 데이터 가져오기
    pub fn get_i8(&mut self) -> i8 {
        if self.index + 1 <= self.bytes.len() {
            let value = self.bytes[self.index];
            self.index += 1;
            value as i8
        }
        else {
            0
        }
    }

    //i16 데이터 가져오기
    pub fn get_i16(&mut self) -> i16 {
        if self.index + 2 <= self.bytes.len() {
            let value = i16::from_be_bytes([self.bytes[self.index]
                , self.bytes[self.index+1]]); //네트워크 바이트오더를 숫자로 
            self.index += 2;
            value
        }
        else {
            0
        }
    }

    //i32 데이터 가져오기
    pub fn get_i32(&mut self) -> i32 {
        if self.index + 4 <= self.bytes.len() {
            let value = i32::from_be_bytes([self.bytes[self.index]
                , self.bytes[self.index+1]
                , self.bytes[self.index+2]
                , self.bytes[self.index+3]]); //네트워크 바이트오더를 숫자로 
            self.index += 4;
            value
        }
        else {
            0
        }
    }

    //i64 데이터 가져오기
    pub fn get_i64(&mut self) -> i64 {
        if self.index + 8 <= self.bytes.len() {
            let value = i64::from_be_bytes([self.bytes[self.index]
                , self.bytes[self.index+1]
                , self.bytes[self.index+2]
                , self.bytes[self.index+3]
                , self.bytes[self.index+4]
                , self.bytes[self.index+5]
                , self.bytes[self.index+6]
                , self.bytes[self.index+7]]); //네트워크 바이트오더를 숫자로 
            self.index += 8;
            value
        }
        else {
            0
        }
    }

    //String 데이터 가져오기
    pub fn get_string(&mut self, length: usize) -> String {
        if self.index + length <= self.bytes.len() {
            let value = std::str::from_utf8(&self.bytes[0..length]).unwrap().to_string();            
            self.index += length;
            value
        }
        else {
            String::new()
        }
    }
}