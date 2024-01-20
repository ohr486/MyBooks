package main

import (
	"fmt"
)

type Status int

const (
	InvalidLogin Status = iota + 1
	NotFound
)

type StatusErr struct {
	Status  Status
	Message string
	Err     error
}

func (se StatusErr) Error() string {
	return se.Message
}

func (se StatusErr) Unwrap() error {
	return se.Err
}

func LoginAndGetData(uid, pwd, file string) ([]byte, error) {
	err := login(uid, pwd)
	if err != nil {
		return nil, StatusErr{
			Status:  InvalidLogin,
			Message: fmt.Sprintf("invalid credentials for user %s", uid),
			Err:     err,
		}
	}
	data, err := getData(file)
	if err != nil {
		return nil, StatusErr{
			Status:  NotFound,
			Message: fmt.Sprintf("file %s not found", file),
			Err:     err,
		}
	}
	return data, nil
}

func main() {
	_, err := LoginAndGetData("[uid]", "[pwd]", "[file]")
	fmt.Println(err)
}

func login(uid, pwd string) error {
	return nil
	// return errors.New("login error")
}

func getData(file string) ([]byte, error) {
	var dummy []byte
	return dummy, nil
	// return nil, fmt.Errorf("getData error: %s", file)
}
