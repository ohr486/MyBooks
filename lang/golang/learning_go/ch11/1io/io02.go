package main

import (
	"compress/gzip"
	"fmt"
	"io"
	"os"
)

func countLetters(r io.Reader) (map[string]int, error) {
	buf := make([]byte, 2048)
	out := map[string]int{}
	for {
		n, err := r.Read(buf)
		for _, b := range buf[:n] {
			if (b >= 'A' && b <= 'Z') || (b >= 'a' && b <= 'z') {
				out[string(b)]++
			}
		}
		if err == io.EOF {
			return out, nil
		}
		if err != nil {
			return nil, err
		}
	}
}

func buildGZipReader(fileName string) (*gzip.Reader, func(), error) {
	r, err := os.Open(fileName)
	if err != nil {
		return nil, nil, err
	}
	gr, err := gzip.NewReader(r)
	if err != nil {
		return nil, nil, err
	}
	return gr, func() {
		gr.Close()
		r.Close()
	}, nil
}

func countLettersInGzipFile() error {
	r, closer, err := buildGZipReader("my_data.txt.gz")
	if err != nil {
		return err
	}
	defer closer()
	counts, err := countLetters(r)
	if err != nil {
		return err
	}
	fmt.Println(counts)
	return nil
}

func main() {
	countLettersInGzipFile()
}
