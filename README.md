# fahrenheit-celsius-converter

---

Simple http Fahrenheit/Celsius/Kelvin converter using [actix-web](https://actix.rs/).

---

> [!NOTE]\
> This is a toy project, not yet finished. It's not recommended to use it in production.


## Usage

From terminal:

```bash
git clone https://github.com/benzlokzik/fahrenheit-celsius-converter/
cd fahrenheit-celsius-converter
cargo build
cargo run
```

Via Docker:

```bash
docker build -t fahrenheit-celsius-converter .
docker run -p 8080:8080 fahrenheit-celsius-converter
```

## API

### Convert Fahrenheit to Celsius

```bash
curl -X GET "http://localhost:8080/convert/f_to_c/100"
```

### Convert Celsius to Fahrenheit

```bash
curl -X GET "http://localhost:8080/convert/c_to_f/100"
```

### Convert Fahrenheit to Kelvin

```bash
curl -X GET "http://localhost:8080/convert/f_to_k/100"
```

### Convert Kelvin to Fahrenheit

```bash
curl -X GET "http://localhost:8080/convert/k_to_f/100"
```

### Convert Celsius to Kelvin

```bash
curl -X GET "http://localhost:8080/convert/c_to_k/100"
```

### Convert Kelvin to Celsius

```bash
curl -X GET "http://localhost:8080/convert/k_to_c/100"
```
