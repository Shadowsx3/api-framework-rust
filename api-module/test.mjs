import { BookingService } from "./index.js";

const service = new BookingService();

const booking = {
  id: 1,
  firstname: "John",
  lastname: "Doe",
  totalprice: 100,
  depositpaid: true,
  bookingdates: {
    checkin: "2023-10-01",
    checkout: "2023-10-05",
  },
  additionalneeds: "Breakfast",
};

const response = await service.addBooking(booking);
console.log(response.data);
