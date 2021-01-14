#include<iostream>
#include<fstream>
#include<string.h>
int main()
{
    char buff[8];
    int level = 1;
    std::fstream f("/home/keeyu/water", std::ios::app | std::ios::in);
    f.seekp(std::ios::end);
    int temp = f.tellp();
    std::cout << temp;
    f.seekp(std::ios::beg);
    if (temp <= 2)
    {
        f << level;
    } else
    {
        f.read(buff, 8);
        f.seekp(std::ios::beg);
        int number = atoi(buff);
        number = 0;
        number++;
        f << number;
        std::cout << "之前喝了:" << buff << "现在一共:" << number << std::endl;
    };
    f.close();
}
