#include <iostream>

class PID
{
private:
    double kp, ki, kd;
    double previous_error;
    double integral;

public:
    PID(double kp, double ki, double kd)
    {
        this->kp = kp;
        this->ki = ki;
        this->kd = kd;
        previous_error = 0.0;
        integral = 0.0;
    }

    double compute(double setpoint, double measured_value, double dt)
    {
        double error = setpoint - measured_value;
        integral += error * dt;
        double derivative = (error - previous_error) / dt;

        double output = (kp * error) + (ki * integral) + (kd * derivative);
        previous_error = error;

        return output;
    }
};
int main()
{
    PID pid(2.0, 0.5, 1.0); // Kp, Ki, Kd values

    double setpoint = 100.0;
    double measured_value = 90.0;
    double dt = 0.1; // time interval in seconds

    for (int i = 0; i < 10; ++i)
    {
        double control = pid.compute(setpoint, measured_value, dt);
        std::cout << "Control Output: " << control << std::endl;

        // Simulate system response (e.g., a motor or temperature change)
        measured_value += control * dt;
        std::cout << "Measured Value: " << measured_value << std::endl;
    }

    return 0;
}
